use crate::auth::passwords::{hash_password, verify_password};

#[test]
fn test_hash_password() {
    let password = "password";
    hash_password(&password).expect("Hashing failed");
}

#[test]
fn test_verify_password() {
    let password = "password";
    let hash = hash_password(&password).expect("Hashing failed");

    let verified = verify_password(&hash, password);
    assert!(verified);
}
