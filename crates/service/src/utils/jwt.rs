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

pub struct Jwt;

impl Jwt {
    pub fn get_key() -> hmac::Hmac<Sha256> {
        let secret_key = var("JWT_SECRET").unwrap_or_else(|_| "default_jwt_secret".to_string());
        Hmac::new_from_slice(secret_key.as_bytes()).unwrap()
    }

    pub fn sign(payload: TokenPayload) -> Result<String, Error> {
        let key = Jwt::get_key();
        let mut claims = BTreeMap::new();

        let mut update_payload = payload.clone();

        update_payload.expire = chrono::Utc::now().timestamp() + update_payload.expire;

        claims.insert(
            "payload",
            serde_json::to_string(&update_payload).map_err(|_| Error::InvalidSignature)?,
        );
        claims.sign_with_key(&key)
    }

    pub fn verify(token: &str) -> Result<TokenPayload, Error> {
        let claims = Self::get_claims(token)?;
        let payload = Self::extract_payload(&claims)?;

        if payload.expire <= chrono::Utc::now().timestamp() {
            return Err(Error::InvalidSignature);
        }

        Ok(payload)
    }

    pub fn extract_info(token: &str) -> Result<TokenPayload, Error> {
        let claims = Self::get_claims(token)?;
        let payload = Self::extract_payload(&claims)?;

        Ok(payload)
    }

    fn get_claims(token: &str) -> Result<BTreeMap<String, String>, Error> {
        token.verify_with_key(&Self::get_key())
    }

    fn extract_payload(claims: &BTreeMap<String, String>) -> Result<TokenPayload, Error> {
        let payload = claims.get("payload").ok_or(Error::InvalidSignature)?;
        serde_json::from_str(payload).map_err(|_| Error::InvalidSignature)
    }
}
