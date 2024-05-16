use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VideoNote {
    pub file_id: String,
    pub file_unique_id: String,
    pub length: u32,
    pub duration: u32,
    pub thumbnail: Option<PhotoSize>,
    pub file_size: Option<u64>,
}
