use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShadowProbeError {
    #[error("HTTP request failed: {0}")]
    HttpError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("AI model error: {0}")]
    AIError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

pub type Result<T> = std::result::Result<T, ShadowProbeError>;
