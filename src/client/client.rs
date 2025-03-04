use crate::types::records::Record;
use super::error::AirtableError;

/// The core Airtable client, responsible for making requests to the API.
pub struct AirtableClient {
    api_key: String,
    base_id: String,
    http_client: reqwest::Client,
}

impl AirtableClient {
    /// Creates a new `AirtableClient` with the given API key and base ID.
    pub fn new(api_key: &str, base_id: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            base_id: base_id.to_string(),
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn list_records(&self, table_name: &str) -> Result<Vec<Record>, AirtableError> {
        crate::endpoints::records::list_records(self, table_name).await
    }

    pub async fn placeholder(&self) -> Result<(), AirtableError> {
        // logic to call airtable endpoints
        Ok(())
    }   

}
