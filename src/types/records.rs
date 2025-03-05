use serde::{Deserialize, Serialize};
use std::fmt;

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

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // You decide how you want the display string to look:
        write!(
            f,
            "Record {{ id: {}, fields: {}, created_time: {:?} }}",
            self.id,
            self.fields,
            self.created_time
        )
    }
}

/// Represents the list of records
#[derive(Debug, Serialize, Deserialize)]
pub struct RecordList {
    pub records: Vec<Record>,

    /// offset for pagination
    pub offset: Option<String>,
}