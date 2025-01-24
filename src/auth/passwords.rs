use bcrypt::{hash, verify, BcryptResult, DEFAULT_COST};

pub fn hash_password(password: &str) -> BcryptResult<String> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    verify(hash, password).unwrap_or(false)
}
