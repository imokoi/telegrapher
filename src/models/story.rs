use serde::{Deserialize, Serialize};

use crate::models::chat::Chat;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Story {
    pub chat: Chat,
    pub id: u64,
}
