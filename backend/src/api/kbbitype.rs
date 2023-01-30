use serde::{Deserialize, Serialize};

type RedisSpellcheckFlat = Vec<(String, String, Vec<(String, String)>)>;

#[derive(Serialize, Deserialize, Debug)]
pub struct KBBIJson {
    pranala: String,
    entri: Vec<KBBIEntri>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KBBIEntri {
    nama: String,
    nomor: String,
    kata_dasar: Vec<String>,
    pelafalan: String,
    bentuk_tidak_baku: Vec<String>,
    varian: Vec<String>,
    makna: Vec<KBBIMakna>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KBBIMakna {
    kelas: Vec<KBBIkelas>,
    submakna: Vec<String>,
    info: String,
    contoh: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KBBIkelas {
    kode: String,
    nama: String,
    deskripsi: String,
}

#[derive(Deserialize, Debug)]
pub struct CekRequest {
    #[serde(default)]
    pub query: String, // the query
    #[serde(default = "ResFormat::default")]
    pub format: ResFormat, // MD or HTML format for corrected words default to MD
    #[serde(default)]
    pub correction: bool, // accept correction from the spellchecker
    #[serde(default = "ToleranceLevel::default")]
    pub tolerance: ToleranceLevel, // Levahstein distance LOW = 1
    #[serde(default)]
    pub english: bool, // whether allow english word which is not exist in kbbi
    #[serde(default)]
    pub tidak_baku: bool, // whether accept suggestion or not
    #[serde(default)]
    pub result_vec: bool,
}

#[derive(Serialize, Debug)]
pub struct CekResponse {
    // #[serde(skip_serializing_if = "String::is_empty")]
    pub result: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub result_vec: Vec<String>,
    pub valid: bool,
    pub reccomendation: RedisSpellcheckFlat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<KBBIJson>,
}

// #[derive(Serialize, Debug)]
// pub struct Reccomendation {
//     entri: String,
//     suggestion: String,
//     note: String,
// }

#[derive(Deserialize, Debug)]
pub enum ResFormat {
    MD,
    HTML,
    NONE,
}
impl ResFormat {
    fn default() -> Self {
        ResFormat::MD
    }
}

#[derive(Deserialize, Debug)]
pub enum ToleranceLevel {
    HIGHEST,
    HIGH,
    MEDIUM,
    LOW,
}
impl ToleranceLevel {
    fn default() -> Self {
        ToleranceLevel::LOW
    }
    pub fn get_number(&self) -> usize {
        // limit high and highest to medium because redisearch is not yet support limiting result
        match self {
            Self::LOW => 1,
            Self::MEDIUM => 2,
            Self::HIGH => 2,
            Self::HIGHEST => 2,
        }
    }
}
