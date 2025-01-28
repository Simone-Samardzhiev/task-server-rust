use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub id: uuid::Uuid,
    pub sub: uuid::Uuid,
    pub exp: i64,
}

impl RefreshTokenClaims {
    pub fn new(id: uuid::Uuid, sub: uuid::Uuid, exp: i64) -> Self {
        Self { id, sub, exp }
    }

    pub fn decode(
        token: &str,
        secret: &str,
    ) -> Result<RefreshTokenClaims, jsonwebtoken::errors::Error> {
        jsonwebtoken::decode::<Self>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )
        .map(|data| data.claims)
    }
    pub fn encode(&self, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
        encode(&self, secret)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: uuid::Uuid,
    pub exp: i64,
}

impl AccessTokenClaims {
    pub fn new(sub: uuid::Uuid, exp: i64) -> Self {
        Self { sub, exp }
    }

    pub fn decode(
        token: &str,
        secret: &str,
    ) -> Result<AccessTokenClaims, jsonwebtoken::errors::Error> {
        jsonwebtoken::decode::<Self>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )
        .map(|data| data.claims)
    }

    pub fn encode(&self, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
        encode(&self, secret)
    }
}

/// Function that will return encoded token.
pub fn encode<T: Serialize>(
    claims: &T,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}
