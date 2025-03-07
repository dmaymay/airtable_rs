use std::env;
use dotenv::dotenv;
use airtable_helper::{
    client::{AirtableClient, AirtableError},
    types::records::Record,
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), AirtableError> {
    dotenv().ok();
    
    let api_key = env::var("AIRTABLE_API_KEY")
        .expect("Expected AIRTABLE_API_KEY to be set");
    let base_id = env::var("AIRTABLE_BASE_ID")
        .expect("Expected AIRTABLE_BASE_ID to be set");
    
    let client = AirtableClient::new(&api_key, &base_id);

    let rec1 = Record {
        id: Some("rec7FUJc4UNF5cimY".to_string()),
        fields: json!({ "Name": "Alice111111", "Status":"Done"}),
        created_time: None,
    };

    let rec2 = Record {
        id: Some("rec2stbejBgiKOceX".to_string()), 
        fields: json!({ "Name": "Bob22222", "Status": "Todo"}),
        created_time: None,
    };

    // let new_records = vec![rec2.clone(); 21];

    let new_records = vec![rec1, rec2];
    let created = client
        .update_records("Table 1", &new_records)
        .await?;

    for record in created {
        println!("Updated record: id={:?}, fields={}", record.id, record.fields);
    }

    Ok(())
}