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
            .service(logout)
            .service(info)
            .service(teams)
            .service(reset_token)
            .service(update_info)
            .service(check),
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

#[post("/logout")]
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

#[get("/teams")]
async fn teams(service: web::Data<Arc<Service>>, token: BearerAuth) -> impl Responder {
    let token_str = token.token();
    let user_id = service
        .auth_service
        .get_user_id_by_token(token_str)
        .await
        .unwrap();

    let res = service
        .team_user_service
        .find_teams_by_user(user_id)
        .await;

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

#[derive(Debug, Deserialize)]
pub struct CheckUsernameQuery {
    email: Option<String>,
    username: Option<String>,
}

#[get("/check")]
async fn check(
    service: web::Data<Arc<Service>>,
    query: web::Query<CheckUsernameQuery>,
) -> impl Responder {
    let payload = query.into_inner();
    let res = match (payload.email, payload.username) {
        (Some(email), Some(username)) => {
            let email_exists = service
                .user_service
                .check_email_exist(&email)
                .await
                .unwrap_or(true);
            let username_exists = service
                .user_service
                .check_username_exist(&username)
                .await
                .unwrap_or(true);
            Ok(email_exists || username_exists)
        }
        (Some(email), None) => service.user_service.check_email_exist(&email).await,
        (None, Some(username)) => service.user_service.check_username_exist(&username).await,
        (None, None) => Ok(true),
    };

    handle_response_by_service(res)
}
