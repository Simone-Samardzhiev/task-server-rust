use serde::Serialize;

/// Struct send by the server containing both tokens.
#[derive(Serialize)]
pub struct TokenGroup {
    access_token: String,
    refresh_token: String,
}

impl TokenGroup {
    /// Method used to create new `TokenGroup`
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Self {
            access_token,
            refresh_token,
        }
    }
}
