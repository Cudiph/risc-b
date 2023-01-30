use actix_web::{error, middleware, web, App, HttpResponse, HttpServer};
use std::env::var;

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let ip_addr = var("APP_ADDRESS").expect("APP_ADDRESS env must be supplied");
    let port = var("APP_PORT").expect("APP_PORT env must be supplied");
    let port: u16 = port.trim().parse().expect("APP_PORT is not a Number");

    let rclient = bb8_redis::RedisConnectionManager::new(var("REDIS_URI").unwrap())
        .expect("cannot create manager");
    let pool = bb8::Pool::builder()
        .max_size(15)
        .build(rclient)
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                error::InternalError::from_response(
                    "",
                    HttpResponse::BadRequest()
                        .content_type("application/json")
                        .body(format!(r#"{{"error":"{}"}}"#, err)),
                )
                .into()
            }))
            .wrap(middleware::Logger::default())
            .service(web::scope("/api").configure(api::config))
    })
    .bind((ip_addr, port))?
    .run()
    .await
}
