use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UserPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct TokenGroup {
    pub refresh_token: String,
    pub access_token: String,
}
