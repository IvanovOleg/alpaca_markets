use thiserror::Error;

#[derive(Error, Debug)]
pub enum AlpacaError {
    #[error("API error: {message}")]
    ApiError { code: Option<u32>, message: String },

    #[error("Authentication failed")]
    AuthenticationError,

    #[error("Rate limit exceeded")]
    RateLimitError,

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("WebSocket error: {0}")]
    WebSocketError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
}

pub type AlpacaResult<T> = Result<T, AlpacaError>;
