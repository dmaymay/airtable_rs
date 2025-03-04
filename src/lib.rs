use dotenv::dotenv;
use reqwest::{self, header, Client};
use serde::Serialize;
use std::{collections::HashMap, env};
use serde_json::{json, Value};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("AIRTABLE_KEY").expect("Missing AIRTABLE_KEY");
    let base_id = env::var("BASE_ID").expect("Missing BASE_ID");
    let table_1_id = env::var("TABLE_1_ID").expect("Missing TABLE_1_ID");
/*     let filtered_view = env::var("FILTERED_VIEW_ID").expect("Missing FILTERED_VIEW_ID");
    let unfiltered_view = env::var("UNFILTERED_VIEW_ID").expect("Missing UNFILTERED_VIEW_ID"); */

    let mut fields = HashMap::new();
    fields.insert("Name".to_string(), json!("New Task"));
    //fields.insert("Status".to_string(), json!("[To Do]"));
    fields.insert("Due".to_string(), json!("2025-03-01T12:00:00.000Z"));
    fields.insert("integer".to_string(), json!(10));
    fields.insert("float".to_string(), json!(3.1415));
    fields.insert("positive_integer".to_string(), json!(7));
    fields.insert("poor_text".to_string(), json!("This is some text."));
    fields.insert("Rich text".to_string(), json!("Rich text example\n\n```\ncode snippet\n```"));

    let record = json!({ "fields": fields });

    match create_record(&base_id, &table_1_id, &api_key, &record).await {
        Ok(_) => println!("Record created successfully!"),
        Err(err) => println!("Error creating record: {}", err),
    }
    
    sleep(Duration::from_secs(3)).await;

    get_records(&base_id, &table_1_id, &api_key, &Some("recbztNx2W30I2eBA".to_string())).await;

    
    //get_records(&base_id, &table_1_id, &api_key, &None);
}

/* fn main() {
    get_request();
    sleep(Duration::from_secs(3)); // Sleep for 3 seconds
    post_request();
} */

async fn get_records(
    base_id: &String,
    table_id: &String,
    api_key: &String,
    record_id: &Option<String>
) -> Result<(), reqwest::Error> {
    let url = match record_id {
        Some(id) => format!("https://api.airtable.com/v0/{}/{}/{}", base_id, table_id, id),
        None => format!("https://api.airtable.com/v0/{}/{}", base_id, table_id),
    };

    let client = Client::new();

    let response = client
        .get(&url)
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .send()
        .await?;

    if response.status().is_success() {
        let body = response.text().await?;
        println!("Response: {}", body);
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}

async fn create_record(
    base_id: &str,
    table_id: &str,
    api_key: &str,
    record: &Value,
) -> Result<(), reqwest::Error> {
    let url = format!("https://api.airtable.com/v0/{}/{}", base_id, table_id);
    
    let client = Client::new();

    let response = client
        .post(&url)
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .header(header::CONTENT_TYPE, "application/json")
        .json(record)
        .send()
        .await?;

    if response.status().is_success() {
        let body = response.text().await?;
        println!("Record Created: {}", body);
    } else {
        println!("Request failed with status: {}", response.status());
        let error_body = response.text().await?;
        println!("Error response: {}", error_body);
    }

    Ok(())
}
