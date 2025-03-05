use super::error::AirtableError;
use crate::endpoints::records::{get_record, list_records};
use crate::types::params::ListRecordsParams;
use crate::types::records::Record;

/// The core Airtable client, responsible for making requests to the API.
pub struct AirtableClient {
    pub api_key: String,
    pub base_id: String,
    pub http_client: reqwest::Client,
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

    pub async fn list_records(
        &self,
        table_name: &str,
        params: Option<ListRecordsParams>,
    ) -> Result<Vec<Record>, AirtableError> {
        let params = params.unwrap_or_default();

        list_records(self, table_name, &params).await
    }

    pub async fn get_record(
        &self,
        table_name: &str,
        record_id: &str,
    ) -> Result<Record, AirtableError> {
        get_record(self, table_name, record_id).await
    }

    pub async fn placeholder(&self) -> Result<(), AirtableError> {
        // logic to call airtable endpoints
        Ok(())
    }
}
