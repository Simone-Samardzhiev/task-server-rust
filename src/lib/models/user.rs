use serde::Deserialize;
use uuid::Uuid;

/// `UserPayload` holds used data from requests.
#[derive(Deserialize)]
struct UserPayload {
    email: String,
    username: String,
    password: String,
}

/// `User` holds used data.
struct User {
    id: i64,
    email: String,
    username: String,
    password: String,
}
