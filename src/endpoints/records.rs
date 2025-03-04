use crate::client::{AirtableClient, AirtableError};

/// List Airtable Records
pub async fn list_records(
    client: &AirtableClient,
    table_name: &str,
) -> Result<(), AirtableError> {
    Ok(())
}