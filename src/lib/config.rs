//! `config` module used to load configuration.
use std::env::{var, VarError};

/// `SERVER_ADDR_KEY` used to access server address.
const SERVER_ADDR_KEY: &str = "SERVER_ADDR";

/// `DATABASE_URL_KEY` used to access database url.
const DATABASE_URL_KEY: &str = "DATABASE_URL";

/// `SECRET_KEY` used to access JWT secret.
const SECRET_KEY: &str = "SECRET";

/// `Config` holds configuration.
pub struct Config {
    /// `server_addr` holds the serer address.
    pub server_addr: String,
    /// `database_url` holds the database url.
    pub database_url: String,
    /// `secret` hold JWT secret
    pub secret: String,
}

impl Config {
    /// `new_from_env` will load the configuration from the environment variables.
    pub fn new_from_env() -> Result<Self, VarError> {
        let server_addr = var(SERVER_ADDR_KEY)?;
        let database_url = var(DATABASE_URL_KEY)?;
        let secret = var("SECRET")?;
        Ok(Self {
            server_addr,
            database_url,
            secret,
        })
    }
}
