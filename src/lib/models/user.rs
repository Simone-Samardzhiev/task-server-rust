use serde::Deserialize;

/// `UserPayload` holds used data from requests.
#[derive(Deserialize)]
pub struct UserPayload {
    pub email: String,
    pub username: String,
    pub password: String,
}

/// `User` holds used data.
pub struct User {
    id: i64,
    email: String,
    username: String,
    password: String,
}
