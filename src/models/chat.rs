use serde::{Deserialize, Serialize};

use crate::models::user::User;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Chat {
    pub id: i64,
    /// Type of the chat, can be either “private”, “group”, “supergroup” or “channel”
    #[serde(rename = "type")]
    pub chat_type: String,
    /// Title, for supergroups, channels and group chats
    pub title: Option<String>,
    /// Username, for private chats, supergroups and channels if available
    pub username: Option<String>,
    /// First name of the other party in a private chat
    pub first_name: Option<String>,
    /// Last name of the other party in a private chat
    pub last_name: Option<String>,
    /// True, if the supergroup chat is a forum (has topics enabled)
    pub is_forum: Option<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatAdministratorRights {
    pub is_anonymous: bool,
    pub can_manage_chat: bool,
    pub can_delete_messages: bool,
    pub can_manage_video_chats: bool,
    pub can_restrict_members: bool,
    pub can_promote_members: bool,
    pub can_change_info: bool,
    pub can_invite_users: bool,
    pub can_post_messages: Option<bool>,
    pub can_edit_messages: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub can_post_stories: Option<bool>,
    pub can_edit_stories: Option<bool>,
    pub can_delete_stories: Option<bool>,
    pub can_manage_topics: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatJoinRequest {
    pub chat: Chat,
    pub from: User,
    pub user_chat_id: u64,
    pub date: u64,
    pub bio: Option<String>,
    pub invite_link: Option<ChatInviteLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatPhoto {
    pub small_file_id: String,
    pub small_file_unique_id: String,
    pub big_file_id: String,
    pub big_file_unique_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatInviteLink {
    pub invite_link: String,
    pub creator: User,
    pub creates_join_request: bool,
    pub is_primary: bool,
    pub is_revoked: bool,
    pub name: Option<String>,
    pub expire_date: Option<u64>,
    pub member_limit: Option<u32>,
    pub pending_join_request_count: Option<u32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatPermissions {
    pub can_send_messages: Option<bool>,
    pub can_send_audios: Option<bool>,
    pub can_send_documents: Option<bool>,
    pub can_send_photos: Option<bool>,
    pub can_send_videos: Option<bool>,
    pub can_send_video_notes: Option<bool>,
    pub can_send_voice_notes: Option<bool>,
    pub can_send_polls: Option<bool>,
    pub can_send_other_messages: Option<bool>,
    pub can_add_web_page_previews: Option<bool>,
    pub can_change_info: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub can_manage_topics: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatLocation {
    pub location: Location,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Birthdate {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}
