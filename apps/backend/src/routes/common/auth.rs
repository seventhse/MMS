use std::sync::Arc;

use crate::common::{handle_response_by_service, ApiResponse, Empty};
use actix_web::{get, post, web, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize};
use service::{
    common::{
        auth_service::{AuthResponse, LoginPayload},
        user_service::{CreateUserDto, UpdateUserDto},
    },
    Service,
};

pub(crate) fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(login)
            .service(register)
            .service(forget)
            .service(info)
            .service(logout)
            .service(reset_token)
            .service(update_info),
    );
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginBody {
    pub email: String,
    pub password: String,
    pub remember: Option<bool>,
}

#[post("/login")]
async fn login(service: web::Data<Arc<Service>>, payload: web::Json<LoginBody>) -> impl Responder {
    let payload = payload.into_inner();
    // TODO: Storeage user login status in redis
    // let remmeber = payload.remember.unwrap_or(false);
    let res = service
        .auth_service
        .login(LoginPayload {
            email: payload.email,
            password: payload.password,
            expire: None,
        })
        .await;
    handle_response_by_service(res)
}

#[post("/register")]
async fn register(
    service: web::Data<Arc<Service>>,
    payload: web::Json<CreateUserDto>,
) -> impl Responder {
    let res = service.auth_service.register(payload.into_inner()).await;
    handle_response_by_service(res)
}

#[get("/logout")]
async fn logout(service: web::Data<Arc<Service>>) -> impl Responder {
    service.auth_service.logout().await;
    ApiResponse::<Empty>::ok(Some("Logout successful!"))
}

#[get("/reset-token")]
async fn reset_token(service: web::Data<Arc<Service>>, token: BearerAuth) -> impl Responder {
    let token_str = token.token();
    let res = service.auth_service.reset_token_by_jwt(token_str).await;
    ApiResponse::<AuthResponse>::success(None, Some(res))
}

#[get("/info")]
async fn info(service: web::Data<Arc<Service>>, token: BearerAuth) -> impl Responder {
    let token_str = token.token();
    let res = service.auth_service.get_user_info_by_token(token_str).await;
    handle_response_by_service(res)
}

#[post("/forget")]
async fn forget() -> impl Responder {
    // TODO: forget password
    ApiResponse::<Empty>::ok(Some("forget successful!"))
}

#[post("/update-info")]
async fn update_info(
    service: web::Data<Arc<Service>>,
    token: BearerAuth,
    payload: web::Json<UpdateUserDto>,
) -> impl Responder {
    let token_str = token.token();
    let res = service
        .auth_service
        .update_user_by_token(token_str, payload.into_inner())
        .await;
    handle_response_by_service(res)
}
