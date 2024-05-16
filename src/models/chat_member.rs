use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberUpdated {
    pub chat: Chat,
    pub from: User,
    pub date: u64,
    pub old_chat_member: ChatMember,
    pub new_chat_member: ChatMember,
    pub invite_link: Option<ChatInviteLink>,
    pub via_chat_folder_invite_link: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum ChatMember {
    Creator(ChatMemberOwner),
    Administrator(ChatMemberAdministrator),
    Member(ChatMemberMember),
    Restricted(ChatMemberRestricted),
    Left(ChatMemberLeft),
    Kicked(ChatMemberBanned),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatMemberOwner {
    pub user: User,
    pub custom_title: Option<String>,
    pub is_anonymous: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatMemberAdministrator {
    pub user: User,
    pub can_be_edited: bool,
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
    pub custom_title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatMemberMember {
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatMemberRestricted {
    pub user: User,
    pub is_member: bool,
    pub can_send_messages: bool,
    pub can_send_audios: bool,
    pub can_send_documents: bool,
    pub can_send_photos: bool,
    pub can_send_videos: bool,
    pub can_send_video_notes: bool,
    pub can_send_voice_notes: bool,
    pub can_send_polls: bool,
    pub can_send_other_messages: bool,
    pub can_add_web_page_previews: bool,
    pub can_change_info: bool,
    pub can_invite_users: bool,
    pub can_pin_messages: bool,
    pub can_manage_topics: bool,
    pub until_date: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatMemberLeft {
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatMemberBanned {
    pub user: User,
    pub until_date: u64,
}
