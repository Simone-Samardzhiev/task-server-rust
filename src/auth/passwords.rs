use bcrypt::{hash, verify, BcryptResult, DEFAULT_COST};

pub fn hash_password(password: &str) -> BcryptResult<String> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap_or(false)
}
