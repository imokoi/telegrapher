use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Voice {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: u32,
    pub mime_type: Option<String>,
    pub file_size: Option<u64>,
}
