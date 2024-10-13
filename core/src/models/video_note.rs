use serde::{Deserialize, Serialize};

use crate::models::photo_size::PhotoSize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VideoNote {
    pub file_id: String,
    pub file_unique_id: String,
    pub length: u64,
    pub duration: u64,
    pub thumbnail: Option<PhotoSize>,
    pub file_size: Option<u64>,
}
