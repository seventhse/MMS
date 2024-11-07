use crate::utils::{
    jwt::{Jwt, TokenPayload},
    DbResult,
};

use super::user_service::{CreateUserDto, PartialUser, UpdateUserDto, UserService};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use std::{env::var, sync::Arc};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
    pub expire: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub expire: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForgetPasswordPayload {
    email: String,
    password: String,
}

pub struct AuthService {
    pub db: Arc<DatabaseConnection>,
    pub user_service: Arc<UserService>,
}

impl AuthService {
    pub fn new(db: Arc<DatabaseConnection>, user_service: Arc<UserService>) -> Self {
        Self { db, user_service }
    }

    pub async fn login(&self, payload: LoginPayload) -> Result<AuthResponse, DbErr> {
        let user = self
            .user_service
            .verify_password_by_email(&payload.email, &payload.password)
            .await?;

        let expire: i64 = payload.expire.unwrap_or_else(|| {
            var("JWT_EXIPRE")
                .ok()
                .and_then(|v| v.parse::<i64>().ok())
                .unwrap_or(1000 * 60 * 10)
        });

        let token = Jwt::sign(TokenPayload {
            user_id: user.user_id,
            expire: expire.clone(),
        })
        .unwrap();

        Ok(AuthResponse { token, expire })
    }

    pub async fn register(&self, payload: CreateUserDto) -> DbResult<AuthResponse> {
        let user = self
            .user_service
            .create_user(payload)
            .await?
            .try_into_model()
            .unwrap();

        // TODO: Send email verify code

        let expire = std::env::var("JWT_EXIPRE")
            .map(|v| v.parse::<i64>().unwrap_or(1000 * 60 * 10))
            .unwrap_or(1000 * 60 * 10);

        let token = Jwt::sign(TokenPayload {
            user_id: user.user_id,
            expire,
        })
        .unwrap();

        Ok(AuthResponse { token, expire })
    }

    pub async fn logout(&self) {
        // TODO: Remove redis data
    }

    pub async fn forget_send_email(&self, email: &str) -> DbResult<bool> {
        let exists = self.user_service.check_email_exist(email).await?;

        if !exists {
            return Err(DbErr::Custom(
                "Cannot find user with provided email".to_string(),
            ));
        }

        // TODO: Send verify mail and save code
        Ok(exists)
    }

    pub async fn forget_verify_code(&self, _email: &str, _code: &str) -> DbResult<bool> {
        // TODO: Check the email and code can match in redix saved data

        Ok(true)
    }

    pub async fn forget(&self, payload: ForgetPasswordPayload) -> DbResult<bool> {
        self.user_service
            .update_password_by_email(&payload.email, &payload.password)
            .await?;

        Ok(true)
    }

    pub async fn get_user_info_by_token(&self, token: &str) -> DbResult<PartialUser> {
        let info = Jwt::extract_info(token).unwrap();
        let user = self.user_service.find_user_by_id(info.user_id).await?;

        Ok(user.unwrap())
    }

    pub async fn update_user_by_token(&self, token: &str, payload: UpdateUserDto) -> DbResult<()> {
        let info = Jwt::extract_info(token).unwrap();
        self.user_service
            .update_user_by_id(info.user_id, payload)
            .await?;

        Ok(())
    }

    pub async fn verify_token(&self, token: &str) -> bool {
        Jwt::verify(token).is_ok()
    }

    pub async fn reset_token_by_jwt(&self, token: &str) -> AuthResponse {
        let info = Jwt::extract_info(token).unwrap();
        let token = Jwt::sign(TokenPayload {
            user_id: info.user_id,
            expire: info.expire.clone(),
        })
        .unwrap();

        AuthResponse {
            token,
            expire: info.expire,
        }
    }
}
