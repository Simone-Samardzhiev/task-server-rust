use chrono::Utc;
use jsonwebtoken::errors::Error as JWTError;
use jsonwebtoken::{DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Struct holding access claims used for access to authorized API points
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: i64,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
}

impl AccessClaims {
    /// Function that will create new `AccessClaims`
    pub fn new(sub: i64, exp: usize, iat: usize, iss: String) -> Self {
        Self { sub, exp, iat, iss }
    }
}

/// Struct holding refresh claims used for revalidating new access token.
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub jti: Uuid,
    pub sub: i64,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
}

impl RefreshClaims {
    /// Function that will create new `RefreshClaims`
    pub fn new(jti: Uuid, sub: i64, exp: usize, iat: usize, iss: String) -> Self {
        Self {
            jti,
            sub,
            exp,
            iat,
            iss,
        }
    }
}

/// Struct used for encoding and decoding tokens.
pub struct Authenticator {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl Authenticator {
    /// Method that will create `Authenticator` with secret used for hasing the tokens.
    pub fn new(secret: String) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    /// Method used to create new `AccessClaims` and hash them into string
    pub fn new_access_token(&self, sub: i64, exp: usize) -> Result<String, JWTError> {
        let claims = AccessClaims::new(
            sub,
            exp,
            Utc::now().timestamp() as usize,
            String::from("task.app.rust"),
        );

        jsonwebtoken::encode(&Header::default(), &claims, &self.encoding_key)
    }

    /// Method used to verify token and transform it into `AccessClaims`
    pub fn verify_access_token(&self, token: &str) -> Result<AccessClaims, JWTError> {
        let token_data = jsonwebtoken::decode::<AccessClaims>(
            token,
            &self.decoding_key,
            &jsonwebtoken::Validation::default(),
        )?;

        Ok(token_data.claims)
    }

    /// Method used to create new `RefreshClaims` and hash them into string.
    pub fn new_refresh_token(&self, jti: Uuid, sub: i64, exp: usize) -> Result<String, JWTError> {
        let claims = RefreshClaims::new(
            jti,
            sub,
            exp,
            Utc::now().timestamp() as usize,
            String::from("task.app.rust"),
        );

        jsonwebtoken::encode(&Header::default(), &claims, &self.encoding_key)
    }

    /// Method used to verify token and transform it into `RefreshClaims`
    pub fn verify_refresh_token(&self, token: &str) -> Result<RefreshClaims, JWTError> {
        let token_data = jsonwebtoken::decode::<RefreshClaims>(
            token,
            &self.decoding_key,
            &jsonwebtoken::Validation::default(),
        )?;

        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Add;

    #[test]
    fn test_encode_access_token() {
        let authenticator = Authenticator::new(String::from("secret"));
        authenticator
            .new_access_token(
                1,
                Utc::now().add(chrono::Duration::minutes(5)).timestamp() as usize,
            )
            .unwrap();
    }

    #[test]
    fn test_encode_refresh_token() {
        let authenticator = Authenticator::new(String::from("secret"));
        authenticator
            .new_refresh_token(
                Uuid::new_v4(),
                1,
                Utc::now().add(chrono::Duration::minutes(5)).timestamp() as usize,
            )
            .unwrap();
    }

    #[test]
    fn test_decode_access_token() {
        let authenticator = Authenticator::new(String::from("secret"));
        let sub = 1;
        let exp = Utc::now().add(chrono::Duration::minutes(5)).timestamp() as usize;
        let token = authenticator.new_access_token(sub, exp).unwrap();
        let claims = authenticator.verify_access_token(&token).unwrap();
        assert_eq!(claims.sub, sub);
        assert_eq!(claims.exp, exp);
    }

    #[test]
    fn test_decode_refresh_token() {
        let authenticator = Authenticator::new(String::from("secret"));
        let jti = Uuid::new_v4();
        let sub = 1;
        let exp = Utc::now().add(chrono::Duration::minutes(5)).timestamp() as usize;
        let token = authenticator.new_refresh_token(jti, sub, exp).unwrap();
        let claims = authenticator.verify_refresh_token(&token).unwrap();
        assert_eq!(claims.jti, jti);
        assert_eq!(claims.sub, sub);
        assert_eq!(claims.exp, exp);
    }
}
