use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Invalid UTF-8: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Filter error: {0}")]
    Filter(String),
    
    #[error("Cache error: {0}")]
    Cache(String),
}

pub type Result<T> = std::result::Result<T, Error>;
