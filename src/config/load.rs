use crate::config::errors::ConfigError;
use std::path::PathBuf;

/// Struct that hold the app data.
pub struct Config {
    /// Database url used to connect to the database.
    pub database_url: String,
    /// The socket to witch the server will listen.
    pub server_socket: String,
    /// The secret used to sign JWT.
    pub jwt_secret: String,
}

impl Config {
    /// Method that will load the config into a struct.
    pub fn load() -> Self {
        let path = find_file().expect("Could not find file");
        load_env(path).expect("Could not load env file");

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let server_socket = std::env::var("SERVER_SOCKET").expect("SERVER_SOCKET must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        Self {
            database_url,
            server_socket,
            jwt_secret,
        }
    }
}

fn load_env(path: PathBuf) -> Result<(), ConfigError> {
    let file = std::fs::read_to_string(path).map_err(|_| ConfigError::FileNotFound)?;

    for line in file.lines() {
        let line = line.trim();
        if line.starts_with("#") || line.is_empty() {
            continue;
        }

        let split: Vec<&str> = line.splitn(2, '=').collect();

        std::env::set_var(split[0], split[1]);
    }

    Ok(())
}
fn find_file() -> Result<PathBuf, ConfigError> {
    let path = std::env::var("CARGO_MANIFEST_DIR")
        .map_err(|_| ConfigError::FileNotFound)?
        .parse::<PathBuf>()
        .map_err(|_| ConfigError::FileNotFound)?;

    if path.join(".env").is_file() {
        return Ok(path.join(".env"));
    }

    Err(ConfigError::FileNotFound)
}
