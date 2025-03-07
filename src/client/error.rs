use reqwest::Response;
use serde_json::Value;
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

pub async fn handle_airtable_error(response: Response, operation_desc: &str) -> AirtableError {
    let status = response.status();
    let text_body = match response.text().await {
        Ok(s) => s,
        Err(e) => {
            return AirtableError::Other(format!(
                "{} failed ({}), also could not read body: {}",
                operation_desc, status, e
            ))
        }
    };

    // Parse as json to get error info
    if let Ok(json_val) = serde_json::from_str::<Value>(&text_body) {
        // Extract type and message from error object if available
        if let Some(error_obj) = json_val.get("error") {
            let err_type = error_obj
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error type");
            let message = error_obj
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("No error message provided");

            return AirtableError::Other(format!(
                "{} failed ({}): {} - {}",
                operation_desc, status, err_type, message
            ));
        }
    }

    // Return raw text in case parsing failed
    AirtableError::Other(format!(
        "{} failed ({}): {}",
        operation_desc, status, text_body
    ))
}
