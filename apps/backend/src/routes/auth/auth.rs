use std::sync::Arc;

use crate::common::{handle_response_by_service, ApiResponse, Empty};
use actix_web::{get, post, web, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use service::{
    common::{
        auth_service::{AuthResponse, LoginPayload},
        user_service::{CreateUserDto, PartialUser},
    },
    Service,
};

pub(crate) fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(login)
            .service(register)
            .service(forget)
            .service(info),
    );
}

#[post("/login")]
async fn login(
    service: web::Data<Arc<Service>>,
    payload: web::Json<LoginPayload>,
) -> impl Responder {
    let res = service.auth_service.login(payload.into_inner()).await;
    handle_response_by_service::<AuthResponse>(res)
}

#[post("/register")]
async fn register(
    service: web::Data<Arc<Service>>,
    payload: web::Json<CreateUserDto>,
) -> impl Responder {
    let res = service.auth_service.register(payload.into_inner()).await;
    handle_response_by_service::<AuthResponse>(res)
}

#[post("/forget")]
async fn forget() -> impl Responder {
    ApiResponse::<Empty>::ok(Some("forget successful!"))
}

#[get("/info")]
async fn info(service: web::Data<Arc<Service>>, token: BearerAuth) -> impl Responder {
    let token_str = token.token();
    let res = service.auth_service.get_user_info_by_token(token_str).await;
    handle_response_by_service::<PartialUser>(res)
}
