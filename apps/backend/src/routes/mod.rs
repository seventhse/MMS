mod common;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    let api = web::scope("/api/v1").configure(init_api_routes);
    cfg.service(api);
}

pub fn init_api_routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(common::init_routes);
}
