use bcrypt::{hash, verify, DEFAULT_COST};
use sha2::{Digest, Sha256};

pub struct PassVerify;

// TODO: Hash salt
impl PassVerify {
    pub fn encrypt_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        hash(password.as_bytes(), DEFAULT_COST)
    }

    pub fn verify_password(password: &str, hashed: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(password.as_bytes(), hashed)
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
