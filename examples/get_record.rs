use std::env;
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
    let record_id = "rec46ly3TlQcVDkkO";
    let table_name = "Table 1";
    let record = client.get_record(table_name, record_id).await?;

    println!("{}", record);

    Ok(())
}