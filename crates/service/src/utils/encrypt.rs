use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, Error, PasswordHash, PasswordVerifier,
};
use sha2::{Digest, Sha256};

pub struct PassVerify;

// TODO: Hash salt
impl PassVerify {
    pub fn encrypt_password(password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string()
    }

    pub fn verify_password(password: &str, hashed: &str) -> Result<bool, Error> {
        let parsed_hash = PasswordHash::new(&hashed).unwrap();
        let res = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);
        match res {
            Ok(_) => Ok(true),
            _ => Ok(false),
        }
    }
}

pub fn generator_unique_id(email: &str) -> String {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let salt = timestamp.to_string();
    let input = format!("{}{}", email, salt);
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}
