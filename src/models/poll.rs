use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PollOption {
    pub text: String,
    pub voter_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PollAnswer {
    pub poll_id: String,
    pub voter_chat: Option<Chat>,
    pub user: Option<Box<User>>,
    pub option_ids: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Poll {
    pub id: String,
    pub question: String,
    pub options: Vec<PollOption>,
    pub total_voter_count: u32,
    pub is_closed: bool,
    pub is_anonymous: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub allows_multiple_answers: bool,
    pub correct_option_id: Option<u8>,
    pub explanation: Option<String>,
    pub explanation_entities: Option<Vec<MessageEntity>>,
    pub open_period: Option<u32>,
    pub close_date: Option<u64>,
}
