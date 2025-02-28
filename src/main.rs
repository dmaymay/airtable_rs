use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let api_key = env::var("AIRTABLE_KEY");
    let base_id = env::var("BASE_ID");
    let table_1_id = env::var("TABLE_1_ID");
    let filtered_view = env::var("FILTERED_VIEW_ID");
    let unfiltered_view = env::var("UNFILTERED_VIEW_ID");

    println!("Access Key {:?}", api_key);
    println!("Base {:?}", base_id);

}
