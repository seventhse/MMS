use std::{collections::BTreeMap, env::var};

use hmac::Hmac;
use jwt::{Error, SignWithKey, VerifyWithKey};
use sea_orm::sqlx::types::chrono;
use serde::{Deserialize, Serialize};
use sha2::{digest::KeyInit, Sha256};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct TokenPayload {
    pub user_id: sea_orm::prelude::Uuid,
    pub expire: i64, // Changed to i64 for timestamp
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct TokenInfo {
    pub user_id: sea_orm::prelude::Uuid,
    pub expire: i64,
    pub sign_time: i64,
}

impl TokenInfo {
    pub fn new(user_id: sea_orm::prelude::Uuid, expire: i64) -> Self {
        let now_timestamp = get_current_timestamp();
        Self {
            user_id,
            expire,
            sign_time: now_timestamp + expire,
        }
    }
}

pub struct Jwt;

fn get_current_timestamp() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

impl Jwt {
    pub fn get_key() -> hmac::Hmac<Sha256> {
        let secret_key = var("JWT_SECRET").unwrap_or_else(|_| "default_jwt_secret".to_string());
        Hmac::new_from_slice(secret_key.as_bytes()).unwrap()
    }

    pub fn sign(payload: TokenPayload) -> Result<String, Error> {
        let key = Jwt::get_key();
        let mut claims = BTreeMap::new();
        let info = TokenInfo::new(payload.user_id, payload.expire);
        log::info!("JWT sign info: \n{:#?}", &info);
        claims.insert(
            "info",
            serde_json::to_string(&info).map_err(|_| Error::InvalidSignature)?,
        );
        claims.sign_with_key(&key)
    }

    pub fn verify(token: &str) -> Result<TokenInfo, Error> {
        let claims = Self::get_claims(token)?;
        let payload = Self::extract_payload(&claims)?;
        let now_timestamp = get_current_timestamp();

        log::info!(
            "Verify payload: \n {:#?} \n Now timestamp: {}",
            &payload,
            &now_timestamp
        );

        if payload.sign_time <= now_timestamp {
            return Err(Error::InvalidSignature);
        }

        Ok(payload)
    }

    pub fn extract_info(token: &str) -> Result<TokenInfo, Error> {
        let claims = Self::get_claims(token)?;
        let payload = Self::extract_payload(&claims)?;

        Ok(payload)
    }

    fn get_claims(token: &str) -> Result<BTreeMap<String, String>, Error> {
        token.verify_with_key(&Self::get_key())
    }

    fn extract_payload(claims: &BTreeMap<String, String>) -> Result<TokenInfo, Error> {
        let info = claims.get("info").ok_or(Error::InvalidSignature)?;
        serde_json::from_str(info).map_err(|_| Error::InvalidSignature)
    }
}
