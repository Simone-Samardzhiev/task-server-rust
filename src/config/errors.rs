use std::fmt::Display;

/// Error that can occur when loading the env file.
#[derive(Debug)]
pub enum ConfigError {
    /// The line is invalid.
    InvalidLine(String),
    /// The file was not found in the project directory.
    FileNotFound,
    /// The file couldn't be read.
    ErrorReadingFile,
    /// The file couldn't be open.
    ErrorOpeningFile,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLine(line) => write!(f, "Invalid line: {}", line),
            Self::FileNotFound => write!(f, "File not found"),
            Self::ErrorReadingFile => write!(f, "Error reading file"),
            Self::ErrorOpeningFile => write!(f, "Error opening file"),
        }
    }
}

