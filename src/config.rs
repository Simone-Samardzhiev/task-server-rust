use envy::from_env;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub address: String,
    pub db_url: String,
}

impl Config {
    pub fn new() -> Self {
        dotenvy::dotenv().expect("Failed to load .env file");
        from_env::<Self>().expect("Failed to parse config")
    }
}
