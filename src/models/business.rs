use super::{chat::Chat, location::Location, sticker::Sticker, user::User};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BusinessConnection {
    pub id: String,
    pub user: User,
    pub user_chat_id: i64,
    pub date: i64,
    pub can_reply: bool,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BusinessMessagesDeleted {
    pub business_connection_id: String,
    pub chat: Chat,
    pub message_ids: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BusinessIntro {
    pub title: String,
    pub message: String,
    pub sticker: Option<Sticker>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BusinessLocation {
    pub address: String,
    pub location: Option<Location>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BusinessOpeningHoursInterval {
    pub opening_minute: i32,
    pub closing_minute: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BusinessOpeningHours {
    pub time_zone_name: String,
    pub opening_hours: Vec<BusinessOpeningHoursInterval>,
}
