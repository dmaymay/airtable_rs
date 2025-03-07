use std::env;
use airtable_helper::types::params::ListRecordsParams;
use dotenv::dotenv;
use airtable_helper::client::AirtableClient;
use airtable_helper::client::AirtableError;

#[tokio::main]
async fn main() -> Result<(), AirtableError> {
    dotenv().ok();

    let api_key = env::var("AIRTABLE_API_KEY")
        .expect("Expected AIRTABLE_API_KEY to be set");
    let base_id = env::var("AIRTABLE_BASE_ID")
        .expect("Expected AIRTABLE_BASE_ID to be set");

    let client = AirtableClient::new(&api_key, &base_id);

    /* let params = ListRecordsParams {
        view: Some("Grid view".to_string()),
        max_records: Some(50),
    }; */

    /* let params = ListRecordsParams {
        view: Some("Grid view".to_string()),
        max_records: Some(3),
        fields: Some(vec!["Name".to_string(), "created".to_string()]),
        sort: Some(("Name".to_string(), "asc".to_string())),
    }; */
    
    let mut params = ListRecordsParams::new();
    /* params.fields = Some(vec!["Name".to_string(), "created".to_string()]);
    params.max_records = Some(4); */
    let table_name = "Table 1";

    let records = client
        .list_records(table_name, Some(params))
        .await?;

    
    println!("Fetched {} records", records.len());
    for record in records {
        println!("Record ID: {:?}", record.id);
        println!("fields: {}", record.fields);
    }

    Ok(())
}