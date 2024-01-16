use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GeneratorError {
    #[error("todo")]
    Random,
    #[error("Invalid API Key")]
    InvalidApiKey,
    #[error("Unknown error")]
    Unknown,

    #[error("Received error message from server: [{}] {}", .status, .message)]
    ResponseContent { status: StatusCode, message: String },

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}
