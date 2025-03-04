use crate::{
    client::{AirtableClient, AirtableError},
    types::records::{RecordList, Record},
};

pub async fn list_records(
    client: &AirtableClient,
    table_name: &str,
) -> Result<Vec<Record>, AirtableError> {
    // get request
    // parse response
    // what to do with offset?
    Ok(vec![])
}