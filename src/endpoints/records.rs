use crate::{
    client::error::handle_airtable_error,
    client::{AirtableClient, AirtableError},
    types::params::ListRecordsParams,
    types::records::{Record, RecordList},
};

use serde_json::{json, Value};

// Fetches all records from a  `table_name` with the given params
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

    // If `max_records` is Some, add `maxRecords=<value>`
    if let Some(mr) = params.max_records {
        base_request = base_request.query(&[("maxRecords", mr.to_string())]);
    }

    // If `view` is Some, add `view=<value>`
    if let Some(ref v) = params.view {
        base_request = base_request.query(&[("view", v)]);
    }

    // If `fields` is Some, add `fields[]=fieldName` for each field
    if let Some(ref fields_vec) = params.fields {
        for field_name in fields_vec {
            base_request = base_request.query(&[("fields[]", field_name)]);
        }
    }

    // If `sort` is Some, add `sort[0][field]` and `sort[0][direction]`
    if let Some((ref sort_field, ref sort_direction)) = params.sort {
        base_request = base_request
            .query(&[("sort[0][field]", sort_field)])
            .query(&[("sort[0][direction]", sort_direction)]);
    }

    // in case of offset
    loop {
        let mut request = base_request
            .try_clone()
            .ok_or_else(|| AirtableError::Other("Failed to clone request".to_string()))?;
        if let Some(ref off) = offset {
            request = request.query(&[("offset", off)]);
        }

        // Get request
        let response = request.send().await?;

        // Return Error in case of non success code
        if !response.status().is_success() {
            let err = handle_airtable_error(response, "List records").await;
            return Err(err);
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

/// Fetches a single record by its `record_id` from the specified `table_name`.
pub async fn get_record(
    client: &AirtableClient,
    table_name: &str,
    record_id: &str,
) -> Result<Record, AirtableError> {
    let url = format!(
        "https://api.airtable.com/v0/{base_id}/{table_name}/{record_id}",
        base_id = client.base_id,
        table_name = table_name,
        record_id = record_id
    );

    // GET request
    let response = client
        .http_client
        .get(&url)
        .header("Authorization", format!("Bearer {}", client.api_key))
        .send()
        .await?;

    // Return Error in case of non success code
    if !response.status().is_success() {
        let err = handle_airtable_error(response, "Get single record").await;
        return Err(err);
    }

    let record: Record = response.json().await?;

    Ok(record)
}

pub async fn create_records(
    client: &AirtableClient,
    table_name: &str,
    records: &[Record],
) -> Result<Vec<Record>, AirtableError> {

    let mut processed = Vec::new();
    let remaining = records.to_vec();

    let mut offset = 0;

    // Build base request
    let url = format!("https://api.airtable.com/v0/{}/{}", client.base_id, table_name);

    while offset < remaining.len() {
        let chunk_end = (offset + 10).min(remaining.len());
        let chunk = &remaining[offset..chunk_end];
        let body = json!({ "records": chunk });

        // POST request
        let response = client
            .http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", client.api_key))
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            // Construct partial success error
            let err_message = handle_airtable_error(response, "Create records").await.to_string();

            // Return an error variant with partial success data
            let processed_count = processed.len();
            return Err(AirtableError::PartialSuccessError {
                processed_count,
                processed,
                remaining: remaining[offset..].to_vec(),
                message: err_message,
            });
        }

        // Success: parse the JSON
        let json_resp: Value = response.json().await?;
        let created_chunk: Vec<Record> = serde_json::from_value(
            json_resp["records"].clone()
        )?;

        // Extend our 'processed' list
        processed.extend(created_chunk);

        // Advance offset to move to the next chunk
        offset = chunk_end;
    }

    // Return the processed records
    Ok(processed)
}


pub async fn update_records(
    client: &AirtableClient,
    table_name: &str,
    records: &[Record],
) -> Result<Vec<Record>, AirtableError> {
    let mut processed = Vec::new();
    let remaining = records.to_vec();
    let mut offset = 0;

    // Build base request
    let url = format!("https://api.airtable.com/v0/{}/{}", client.base_id, table_name);

    while offset < remaining.len() {
        let chunk_end = (offset + 10).min(remaining.len());
        let chunk = &remaining[offset..chunk_end];
        let body = json!({ "records": chunk });

        // PATCH request
        let response = client
            .http_client
            .patch(&url)
            .header("Authorization", format!("Bearer {}", client.api_key))
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            // Construct partial success error
            let err_message = handle_airtable_error(response, "Update records").await.to_string();
            let processed_count = processed.len();
            // Return an error variant with partial success data
            return Err(AirtableError::PartialSuccessError {
                processed_count,
                processed,
                remaining: remaining[offset..].to_vec(),
                message: err_message,
            });
        }

        // Success: parse the JSON
        let json_resp: Value = response.json().await?;
        let updated_chunk: Vec<Record> =
            serde_json::from_value(json_resp["records"].clone())?;

        // Extend our 'processed' list
        processed.extend(updated_chunk);

        // Advance offset to move to the next chunk
        offset = chunk_end;
    }

    // Return the processed records
    Ok(processed)
}
