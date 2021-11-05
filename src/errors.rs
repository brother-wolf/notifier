use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ConfigError {
    IoError(std::io::Error),
    ParseError(serde_json::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::IoError(e) => write!(f, "{}", e),
            ConfigError::ParseError(e) => write!(f, "{}", e),
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConfigError::ParseError(e) => Some(e),
            ConfigError::IoError(e) => Some(e),
        }
    }
}