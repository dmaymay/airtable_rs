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

    // Sample records
    let rec1 = Record {
        id: None,
        fields: json!({ "Name": "Alice", "Status": 2 }),
        created_time: None,
    };
    let rec2 = Record {
        id: None, 
        fields: json!({ "Name": "Bob" }),
        created_time: None,
    };
    let new_records = vec![rec2.clone(), rec1.clone()];

    // We'll keep track of both created and remaining records
    let mut created_records: Option<Vec<Record>> = None;
    let mut remaining_records: Option<Vec<Record>> = None;

    match client.create_records("Table 1", &new_records).await {
        // 1) Full success
        Ok(created) => {
            println!("All records created successfully! Count: {}", created.len());
            // Store them in `fully_created_records` for later usage
            created_records = Some(created);
        }
        // 2) Partial success
        Err(AirtableError::PartialSuccessError {
            processed,
            processed_count,
            remaining,
            message,
        }) => {
            eprintln!("Partial success: {processed_count} records created so far.");
            eprintln!("Error message: {message}");
            eprintln!("{} records were NOT created yet.", remaining.len());

            // The chunk that succeeded
            created_records = Some(processed);

            // The chunk that failed plus any future unprocessed
            remaining_records = Some(remaining);
        }
        // 3) Other errors (no partial success data)
        Err(e) => {
            eprintln!("Create records failed entirely, no partial results: {e}");
        }
    }

    // Now we can use both variables if needed

    if let Some(created) = &created_records {
        for record in created {
            println!("Created record: id={:?}, fields={}", record.id, record.fields);
        }
    }

    if let Some(remaining) = &remaining_records {
        eprintln!(
            "We have {} remaining records to possibly retry!",
            remaining.len()
        );
    }

    Ok(())
}