mod auth;
mod user;

use actix_web::web;

pub(crate) fn init_routes(cfg: &mut web::ServiceConfig) {
    auth::init_routes(cfg);
    user::init_routes(cfg);
}
