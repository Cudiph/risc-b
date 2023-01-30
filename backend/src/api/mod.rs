use actix_web::web;

pub mod kbbitype;
mod v1;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(v1::ping)
        .service(v1::post_cek)
        .service(v1::get_cek)
        .service(v1::opts_cek);
}
