use serde_json::json;
use std::env;
use dotenv::dotenv;
use airtable_helper::{
    client::{AirtableClient, AirtableError},
    types::records::Record,
};

#[tokio::main]
async fn main() -> Result<(), AirtableError> {
    dotenv().ok();
    let api_key = env::var("AIRTABLE_API_KEY")
        .expect("Expected AIRTABLE_API_KEY to be set");
    let base_id = env::var("AIRTABLE_BASE_ID")
        .expect("Expected AIRTABLE_BASE_ID to be set");
    
    let client = AirtableClient::new(&api_key, &base_id);
    let table_name = "Table 1";

    // New record
    let new_rec = Record {
        id: None,
        fields: json!({ "Name": "New Person", "poor_text": "25" }),
        created_time: None,
    };

    // Existing record
    let existing_rec = Record {
        id: Some("rec2stbejBgiKOceX".to_string()),
        fields: json!({ "poor_text": "24" }), 
        created_time: None,
    };

    // Upload (create or update) in a single call
    let records = client
        .upload_records(table_name, &[new_rec, existing_rec])
        .await?;

    for rec in records {
        println!("Record ID: {:?}, fields: {}", rec.id, rec.fields);
    }

    Ok(())
}