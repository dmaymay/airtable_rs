#[derive(Debug)]
pub struct ListRecordsParams {
    pub view: Option<String>,
    pub max_records: Option<u32>,
}

// Provide default values for these optional fields.
impl Default for ListRecordsParams {
    fn default() -> Self {
        Self {
            view: None,
            max_records: None,
        }
    }
}