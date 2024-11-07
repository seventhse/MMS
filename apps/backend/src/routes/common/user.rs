use std::sync::Arc;

use crate::common::handle_response_by_service;
use actix_web::{get, web, Responder};
use service::{common::user_service::PartialUser, Service};

pub(crate) fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").service(get_all));
}

#[get("/all")]
async fn get_all(service: web::Data<Arc<Service>>) -> impl Responder {
    let res = service.user_service.find_user_all().await;

    handle_response_by_service::<Vec<PartialUser>>(res)
}
