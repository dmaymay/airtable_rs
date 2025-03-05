use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a single record
#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    /// The unique ID for this record.
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub id: Option<String>,
    
    // for now fields as json 
    pub fields: serde_json::Value,

    /// Airtable generated value -> createdTime
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "createdTime")]
    pub created_time: Option<String>,
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Record {{ id: {}, fields: {}, created_time: {:?} }}",
            self.id.clone().unwrap_or("".to_string()),
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