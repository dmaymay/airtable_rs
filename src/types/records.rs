use serde::{Deserialize, Serialize};

/// Represents a single record
#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    /// The unique ID for this record.
    pub id: String,
    // for now fields as json 
    pub fields: serde_json::Value,

    /// Airtable generated value -> createdTime
    #[serde(rename = "createdTime")]
    pub created_time: Option<String>,
}

/// Represents the list of records
#[derive(Debug, Serialize, Deserialize)]
pub struct RecordList {
    pub records: Vec<Record>,

    /// offset for pagination
    pub offset: Option<String>,
}