use crate::auth::jwt::*;

static SECRET: &str = "SECRET";

#[test]
fn test_refresh_token() {
    let id = uuid::Uuid::new_v4();
    let sub = uuid::Uuid::new_v4();

    let refresh_claims = RefreshTokenClaims::new(id, sub);

    let token = encode(&refresh_claims, SECRET);
    assert!(token.is_ok());
    let token = token.unwrap();

    let decoded = RefreshTokenClaims::decode(&token, SECRET);
    assert!(decoded.is_ok());
    let decoded = decoded.unwrap();

    assert_eq!(refresh_claims.id, decoded.id);
    assert_eq!(refresh_claims.sub, decoded.sub);
    assert_eq!(refresh_claims.exp, decoded.exp);
}

#[test]
fn test_access_token() {
    let sub = uuid::Uuid::new_v4();

    let access_claims = AccessTokenClaims::new(sub);
    let token = encode(&access_claims, SECRET);
    assert!(token.is_ok());
    let token = token.unwrap();

    let decoded = AccessTokenClaims::decode(&token, SECRET);
    assert!(decoded.is_ok());
    let decoded = decoded.unwrap();

    assert_eq!(access_claims.sub, decoded.sub);
    assert_eq!(access_claims.exp, decoded.exp);
}
