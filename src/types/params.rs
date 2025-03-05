#[derive(Debug)]
pub struct ListRecordsParams {
    pub view: Option<String>,
    pub max_records: Option<u32>,
    pub fields: Option<Vec<String>>,
    pub sort: Option<(String, String)>,
}

// Default values for these optional params.
impl Default for ListRecordsParams {
    fn default() -> Self {
        Self {
            view: None,
            max_records: None,
            sort: None,
            fields: None,
        }
    }
}

impl ListRecordsParams {
    /// `new()` that just returns the default instance
    pub fn new() -> Self {
        Self::default()
    }
}
