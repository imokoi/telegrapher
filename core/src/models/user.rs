use serde::{Deserialize, Serialize};

use crate::models::photo_size::PhotoSize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    /// user id
    pub id: u64,
    /// if the user is a bot
    pub is_bot: bool,
    /// user first name
    pub first_name: String,
    /// user last name
    pub last_name: Option<String>,
    /// username
    pub username: Option<String>,
    /// user language code
    pub language_code: Option<String>,
    /// True, if this user is a Telegram Premium user
    pub is_premium: Option<bool>,
    /// True, if this user added the bot to the attachment menu
    pub added_to_attachment_menu: Option<bool>,
    /// True, if the bot can be invited to groups. Returned only in getMe.
    pub can_join_groups: Option<bool>,
    /// True, if privacy mode is disabled for the bot. Returned only in getMe.
    pub can_read_all_group_messages: Option<bool>,
    /// True, if the bot supports inline queries. Returned only in getMe.
    pub supports_inline_queries: Option<bool>,
    /// True, if the bot can be connected to a Telegram Business account to receive its messages. Returned only in getMe.
    pub can_connect_to_business: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserProfilePhotos {
    pub total_count: u32,
    pub photos: Vec<Vec<PhotoSize>>,
}
