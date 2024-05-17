use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct File {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: Option<u64>,
    pub file_path: Option<String>,
}
