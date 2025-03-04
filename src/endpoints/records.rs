use crate::{
    client::{AirtableClient, AirtableError},
    types::params::ListRecordsParams,
    types::records::{Record, RecordList},
};
pub async fn list_records(
    client: &AirtableClient,
    table_name: &str,
    params: &ListRecordsParams,
) -> Result<Vec<Record>, AirtableError> {
    let mut all_records = Vec::new();
    let mut offset: Option<String> = None;

    // Build base request
    let base_url = format!(
        "https://api.airtable.com/v0/{base_id}/{table_name}",
        base_id = client.base_id,
        table_name = table_name
    );

    let mut base_request = client
        .http_client
        .get(&base_url)
        .header("Authorization", format!("Bearer {}", client.api_key));

    // If `params.max_records` is Some, add it
    if let Some(mr) = params.max_records {
        base_request = base_request.query(&[("maxRecords", mr.to_string())]);
    }

    // If `params.view` is Some, add it
    if let Some(ref v) = params.view {
        base_request = base_request.query(&[("view", v)]);
    }

    // in case of offset
    loop {
        let mut request = base_request
            .try_clone()
            .ok_or_else(|| AirtableError::Other("Failed to clone request".to_string()))?;
        if let Some(ref off) = offset {
            request = request.query(&[("offset", off)]);
        }

        // Send the request
        let response = request.send().await?;

        // Return Error in case of non success code
        if !response.status().is_success() {
            return Err(AirtableError::Other(format!(
                "Error status code: {}",
                response.status()
            )));
        }

        let record_list: RecordList = response.json().await?;
        all_records.extend(record_list.records);

        if let Some(off) = record_list.offset {
            offset = Some(off);
        } else {
            break;
        }
    }

    Ok(all_records)
}
