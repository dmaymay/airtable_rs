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
