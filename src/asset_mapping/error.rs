use thiserror::Error;

#[derive(Error, Debug)]
pub enum AssetMappingError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("JSON parse error: {0}")]
    JsonParse(String),
    #[error("API error (status {status_code}): {message}")]
    Api { status_code: u16, message: String },
}
