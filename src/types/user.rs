use serde::Deserialize;
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
