use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Story {
    pub chat: Chat,
    pub id: u64,
}
