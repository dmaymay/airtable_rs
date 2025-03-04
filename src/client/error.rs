use thiserror::Error;

// General error handling
#[derive(Debug, Error)]
pub enum AirtableError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Other error occurred: {0}")]
    Other(String),
}