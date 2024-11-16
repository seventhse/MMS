mod auth;
mod user;
mod team;

use actix_web::web;

pub(crate) fn init_routes(cfg: &mut web::ServiceConfig) {
    auth::init_routes(cfg);
    user::init_routes(cfg);
    team::init_routes(cfg);
}
