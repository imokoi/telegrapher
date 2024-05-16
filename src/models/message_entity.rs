use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageEntity {
    #[serde(rename = "type")]
    pub type_field: String,
    pub offset: u16,
    pub length: u16,
    pub url: Option<String>,
    pub user: Option<User>,
    pub language: Option<String>,
    pub custom_emoji_id: Option<String>,
}
