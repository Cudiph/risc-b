use super::kbbitype::{CekRequest, CekResponse, ResFormat};
use actix_web::http::StatusCode;
use actix_web::{get, options, post, web, HttpRequest, HttpResponse};
use bb8::{Pool, PooledConnection};
use bb8_redis::RedisConnectionManager;
use redis::JsonAsyncCommands;
use redis::{cmd, from_redis_value, Value};
use std::collections::VecDeque;

type ConnPool = Pool<RedisConnectionManager>;
type RedisSpellcheck = Vec<Vec<(String, String, Vec<Vec<(String, String)>>)>>;
type RedisSpellcheckFlat = Vec<(String, String, Vec<(String, String)>)>;

// should added in proxy instead if using cors
// const ALLOWED_ORIGIN: &'static str = "*";

/// check database connection
#[get("/v1/ping")]
pub async fn ping(req: HttpRequest) -> HttpResponse {
    let mut conn = req.app_data::<ConnPool>().unwrap().get().await.unwrap();
    let pong: String = cmd("PING").query_async(&mut *conn).await.unwrap();
    match pong.as_str() {
        "PONG" => HttpResponse::Ok().body(pong),
        _ => HttpResponse::Ok().body("Error"),
    }

    // HttpResponse::Ok().body("ceker")
}

/// cors preflight
#[options("/v1/cek")]
pub async fn opts_cek() -> HttpResponse {
    HttpResponse::Ok()
        .status(StatusCode::NO_CONTENT)
        // .insert_header(("Access-Control-Allow-Origin", ALLOWED_ORIGIN))
        .insert_header(("Access-Control-Allow-Methods", "POST, GET"))
        .insert_header(("Access-Control-Max-Age", "86400"))
        .insert_header(("Access-Control-Allow-Headers", "content-type"))
        .finish()
}

/// specs for using cek api
#[get("/v1/cek")]
pub async fn get_cek() -> HttpResponse {
    let spesification = r#"
        use POST method to use this API endpoint
        ================== request payload =====================
        query: string
        english: boolean
        correction: boolean
        tidak_baku: boolean
        result_vec: boolean
        format: "MD" | "HTML" | "NONE"
        tolerance: "LOW" | "MEDIUM" | "HIGH" | "HIGHEST"
        ========================================================
        query parameter is required, the rest is optional
    "#;
    HttpResponse::Ok().body(spesification)
}

// Possible response are:
// idk man i'm too lazy to write the specs
// lets puts it to todo!() hohoho
#[post("/v1/cek")]
pub async fn post_cek(payload: web::Json<CekRequest>, req: HttpRequest) -> HttpResponse {
    // println!("{payload:?}");

    let query = payload.query.trim();
    if query == "" {
        return HttpResponse::BadRequest()
            // .insert_header(("Access-Control-Allow-Origin", ALLOWED_ORIGIN))
            .content_type("application/json")
            .body(r#"{"error": "query tidak boleh kosong"}"#);
    }

    if query.len() > 20000 {
        return HttpResponse::BadRequest()
            // .insert_header(("Access-Control-Allow-Origin", ALLOWED_ORIGIN))
            .content_type("application/json")
            .body(r#"{"error": "teks tidak boleh lebih dari 20,000 karakter"}"#);
    }

    let mut conn = req.app_data::<ConnPool>().unwrap().get().await.unwrap();

    let mut ok = HttpResponse::Ok();
    // ok.insert_header(("Access-Control-Allow-Origin", ALLOWED_ORIGIN));

    // first stage check if whole sentence is exist in dictionary
    // if it exist -> return the response with the detail of the term
    let whole_sentence = check_whole_sentence(&mut conn, query).await;
    if let Some(rawjson) = whole_sentence {
        let detailjson = serde_json::from_str(&rawjson[1..(rawjson.len() - 1)]).unwrap();
        let res = CekResponse {
            valid: true,
            result: String::new(),
            result_vec: Vec::new(),
            detail: detailjson,
            reccomendation: Vec::new(),
        };
        return ok.json(res);
    }

    let escaped_query = remove_tokenization(query);

    let distance = payload.tolerance.get_number().to_string();
    let mut spellcheck_args = vec!["kbbi", &escaped_query, "DISTANCE", &distance];

    if payload.english {
        spellcheck_args
            .extend_from_slice(&["TERMS", "EXCLUDE", "english", "TERMS", "INCLUDE", "english"]);
    }

    if payload.tidak_baku {
        spellcheck_args.extend_from_slice(&[
            "TERMS",
            "EXCLUDE",
            "tidak_baku",
            "TERMS",
            "INCLUDE",
            "tidak_baku",
        ]);
    }

    // second stage is get the wrong term and then check for base word
    let reccomendation: RedisSpellcheck = cmd("FT.SPELLCHECK")
        .arg(&spellcheck_args)
        .query_async(&mut *conn)
        .await
        .unwrap();

    let mut reccomendation_flat = flatten_dikit(reccomendation);

    let splitted = split_but_keep_delimiter(query);
    let result = mark_invalid_case(splitted, &mut reccomendation_flat);

    let valid_response = CekResponse {
        valid: true,
        detail: None,
        result: String::new(),
        result_vec: Vec::new(),
        reccomendation: Vec::new(),
    };

    // valid if no term is wrong
    if reccomendation_flat.is_empty() {
        return ok.json(valid_response);
    }

    // affix checking by testing the base term (kata dasar checking)
    let mut reccomendation_flat_new: RedisSpellcheckFlat = Vec::new();
    let mut reccomendation_flat_new_end = Vec::new();
    for i in reccomendation_flat {
        if i.0 == "CASES" {
            reccomendation_flat_new_end.push(i);
            continue;
        }

        let wrong_term = &i.1;
        let exist = base_word_exist_check(wrong_term, &mut conn).await;

        if !exist {
            reccomendation_flat_new.push(("TERM".to_string(), wrong_term.to_string(), i.2));
        }
    }

    reccomendation_flat_new.extend(reccomendation_flat_new_end);

    // valid if no term is wrong
    if reccomendation_flat_new.is_empty() {
        return ok.json(valid_response);
    }

    limit_reccomendation_to_n_words(&mut reccomendation_flat_new, 25);
    let result = mark_invalid_term(result, &mut reccomendation_flat_new, &payload);

    if payload.result_vec {
        let finalresponse = CekResponse {
            valid: false,
            detail: None,
            result: String::new(),
            result_vec: result,
            reccomendation: reccomendation_flat_new,
        };
        return ok.json(finalresponse);
    }

    let finalresponse = CekResponse {
        valid: false,
        detail: None,
        result: result.join(""),
        result_vec: Vec::new(),
        reccomendation: reccomendation_flat_new,
    };

    ok.json(finalresponse)
}

/// limit the spellcheck reccomendation for saving network bandwidth
/// don't really affect performance
/// see: https://github.com/RediSearch/RediSearch/issues/969
fn limit_reccomendation_to_n_words(rec: &mut RedisSpellcheckFlat, n: usize) {
    for i in rec {
        i.2.truncate(n);
    }
}

/// reference: https://redis.io/docs/stack/search/reference/escaping/
fn remove_tokenization(query: &str) -> String {
    let illegal_chars = vec![
        '"', ':', '~', '%', '(', ')', ':', ';', '-', '|', '{', '}', '[', ']', '\\', '<', '>', '=',
        '*',
    ];

    query
        .chars()
        .map(|e| match e {
            e if illegal_chars.contains(&e) => ' ',
            _ => e,
        })
        .collect()
}

/// escape instead of remove
fn escape_tokenization(query: &str) -> String {
    let illegal_chars = vec![
        '"', ':', '~', '%', '(', ')', ':', ';', '-', '|', '{', '}', '[', ']', '\\', '<', '>', '=',
        '*',
    ];

    let mut bruh = String::new();

    for i in query.chars() {
        if illegal_chars.contains(&i) {
            bruh.push_str(format!("\\{i}").as_str());
        } else {
            bruh.push(i);
        }
    }

    bruh
}

/// check if all chars in string is numeric
fn is_all_number(text: &String) -> bool {
    for i in text.chars() {
        if !i.is_numeric() {
            return false;
        }
    }

    true
}

/// mark word if [`verify_cases`] false
///
/// [`verify_cases`]: v1::verifiy_cases
fn mark_invalid_case(
    query_vec: Vec<String>,
    reccomendations: &mut RedisSpellcheckFlat,
) -> Vec<String> {
    for i in (0..query_vec.len()).step_by(2) {
        if !verify_cases(&query_vec[i]) {
            let sugg = (
                "CASES".to_string(),
                query_vec[i].clone(),
                vec![("1".to_string(), "Penulisan Kapital tidak valid".to_string())],
            );
            reccomendations.push(sugg);
        }
    }

    query_vec
}

/// true if valid, false otherwise
/// LASER > valid
/// Laser > valid
/// laser > valid
/// laSeR > invalid
/// LaSeR > invalid
fn verify_cases(text: &String) -> bool {
    let mut uppercase = 0;
    let mut _lowercase = 0;
    let len = text.len();

    for i in text.chars() {
        if i.is_lowercase() {
            _lowercase += 1;
        } else {
            uppercase += 1;
        }
    }

    if uppercase == 0 || uppercase == len {
        true
    } else if text.chars().next().unwrap().is_uppercase() && uppercase == 1 {
        true
    } else {
        false
    }
}

/// Mark invalid term to prefered configuration
///
/// # Example
///
/// in "ini adalah contohh" sentence the word contohh is not valid
/// the the resulting value would be "ini adalah **contohh**"
/// if the person choose Markdown format
fn mark_invalid_term(
    query_vec: Vec<String>,
    reccomendations: &mut RedisSpellcheckFlat,
    pload: &CekRequest,
) -> Vec<String> {
    let mut query_vec = query_vec;

    let affix = match pload.format {
        ResFormat::NONE => ("", ""),
        ResFormat::MD => ("**", "**"),
        ResFormat::HTML => ("<b>", "</b>"),
    };

    let mut word_idx = 0;
    // println!("before: {vec:?}");

    let mut to_be_removed = VecDeque::new();

    for i in 0..reccomendations.len() {
        let mut recc = &mut reccomendations[i];
        let term = &recc.1;

        if is_all_number(term) {
            to_be_removed.push_front(i);
            continue;
        }

        for (j, val) in query_vec[word_idx..].iter().enumerate() {
            if term != &val.to_lowercase() {
                continue;
            }
            recc.1 = val.to_string();
            word_idx += j;

            if let Some(el) = query_vec.get_mut(word_idx) {
                let the_else = (String::new(), el.to_string());
                if pload.correction {
                    *el = format!(
                        "{}{}{}",
                        affix.0,
                        recc.2.get(0).unwrap_or_else(|| &the_else).1,
                        affix.1
                    )
                } else {
                    *el = format!("{}{el}{}", affix.0, affix.1)
                }
            }

            break;
        }
    }

    for i in to_be_removed {
        reccomendations.remove(i);
    }

    // println!("after: {vec:?}");
    query_vec
}

/// the delimiter here is non-alphanumeric characters
///
/// # Example:
///
/// ```
/// assert!(vec!["Ze", " " "End", " ", "Of", " ", The", "\n", "Line"], fun("Ze End Of The\nLine""));
/// ````
fn split_but_keep_delimiter(text: &str) -> Vec<String> {
    let mut the_vec = Vec::new();

    let mut is_alpha_state = true;
    let mut prepared_string = String::new();

    for i in text.chars() {
        if i.is_alphanumeric() && !is_alpha_state || !i.is_alphanumeric() && is_alpha_state {
            is_alpha_state = !is_alpha_state;
            the_vec.push(prepared_string.clone());
            prepared_string.clear();
        }

        prepared_string.push(i);
    }

    the_vec.push(prepared_string.clone());

    the_vec
}

/// Check the whole sentence is exist in database
async fn check_whole_sentence<'a>(
    conn: &mut PooledConnection<'a, RedisConnectionManager>,
    text: &str,
) -> Option<String> {
    let search = cmd("FT.SEARCH")
        .arg(&[
            "kbbi",
            format!("@key:{{{}}}", escape_tokenization(text)).as_str(),
            "NOCONTENT",
        ])
        .query_async(&mut **conn)
        .await;
    let search: Vec<Value> = match search {
        Ok(res) => res,
        Err(_) => return None,
    };
    let count: u8 = from_redis_value(&search[0]).unwrap();
    if count == 0 {
        return None;
    }

    let key: String = from_redis_value(&search[count as usize]).unwrap();
    let doc: String = conn.json_get(key, "$").await.unwrap();

    Some(doc)
}

/// Flatten the spellcheck response from redis for easier iteration and debugging
fn flatten_dikit(spellcheck_res: RedisSpellcheck) -> RedisSpellcheckFlat {
    let mut base: RedisSpellcheckFlat = Vec::new();
    for i in spellcheck_res {
        let mut sugbase: Vec<(String, String)> = Vec::new();
        let (termstr, term, sugv) = &i[0];
        for j in sugv {
            let sugtuple = &j[0];
            sugbase.push(sugtuple.clone());
        }

        base.push((termstr.to_string(), term.to_string(), sugbase));
    }

    base
}

/// remove prefix from a term
fn get_no_prefix(text: &str) -> Vec<String> {
    let prefix = [
        "meng", "di", "ber", "ter", "ke", "peng", "per", "pe", "se", "pen", "men",
    ];
    let mut no_prefix_vec = Vec::new();
    for p in prefix {
        if text.starts_with(p) {
            let no_prefix = &text[(p.len())..];
            no_prefix_vec.push(no_prefix.to_string());
        }
    }

    no_prefix_vec
}

/// remove suffix from a term
fn get_no_suffix(text: &str) -> Vec<String> {
    let suffix = [
        // native indonesian suffix
        "an", "in", "kan", "ku", "mu", "nya", "kah", "lah", "tah",
        "pun", // "i" // if "i" is enabled it will trigger many false negative
              // naturalization suffix
              // "isme", "graf", "grafi", "man", "wan", "wati",
    ];

    let mut no_suffix_vec = Vec::new();

    for s in suffix {
        if text.ends_with(s) {
            let no_suffix = &text[..(text.len() - s.len())];
            no_suffix_vec.push(no_suffix.to_string());
        }
    }

    no_suffix_vec
}

/// search the base word return true if it exist in the dict
async fn base_word_exist_check<'a>(
    text: &String,
    conn: &mut PooledConnection<'a, RedisConnectionManager>,
) -> bool {
    let mut possible_base_word = Vec::new();

    let no_prefix = get_no_prefix(text);
    for i in &no_prefix {
        let no_prefix_nor_suffix = get_no_suffix(&i);
        possible_base_word.extend(no_prefix_nor_suffix);
    }

    let no_suffix = get_no_suffix(text);
    for i in &no_suffix {
        let no_suffix_nor_prefix = get_no_prefix(&i);
        possible_base_word.extend(no_suffix_nor_prefix);
    }

    possible_base_word.extend(no_prefix);
    possible_base_word.extend(no_suffix);

    if possible_base_word.is_empty() {
        return false;
    }

    let key_query = format!("@key:{{{}}}", possible_base_word.join("|"));

    let res = cmd("FT.SEARCH")
        .arg(&["kbbi", key_query.as_str(), "NOCONTENT"])
        .query_async(&mut **conn)
        .await;

    let res: Vec<Value> = match res {
        Ok(res) => res,
        Err(_) => return false,
    };

    let count: u64 = from_redis_value(&res[0]).unwrap();
    match count {
        0 => false,
        1.. => true,
    }
}
