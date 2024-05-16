use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SwitchInlineQueryChosenChat {
    pub query: Option<String>,
    pub allow_user_chats: Option<bool>,
    pub allow_bot_chats: Option<bool>,
    pub allow_group_chats: Option<bool>,
    pub allow_channel_chats: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,
    pub is_persistent: Option<bool>,
    pub resize_keyboard: Option<bool>,
    pub one_time_keyboard: Option<bool>,
    pub input_field_placeholder: Option<String>,
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeyboardButton {
    pub text: String,
    pub request_users: Option<KeyboardButtonRequestUsers>,
    pub request_chat: Option<KeyboardButtonRequestChat>,
    pub request_contact: Option<bool>,
    pub request_location: Option<bool>,
    pub request_poll: Option<KeyboardButtonPollType>,
    pub web_app: Option<WebAppInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeyboardButtonRequestUsers {
    pub request_id: i32,
    pub user_is_bot: Option<bool>,
    pub user_is_premium: Option<bool>,
    pub max_quantity: Option<u32>,
    pub request_name: Option<bool>,
    pub request_username: Option<bool>,
    pub request_photo: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeyboardButtonRequestChat {
    pub request_id: i32,
    pub chat_is_channel: bool,
    pub chat_is_forum: Option<bool>,
    pub chat_has_username: Option<bool>,
    pub chat_is_created: Option<bool>,
    pub user_administrator_rights: Option<ChatAdministratorRights>,
    pub bot_administrator_rights: Option<ChatAdministratorRights>,
    pub bot_is_member: Option<bool>,
    pub request_title: Option<bool>,
    pub request_username: Option<bool>,
    pub request_photo: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeyboardButtonPollType {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplyKeyboardRemove {
    pub remove_keyboard: bool,
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ForceReply {
    pub force_reply: bool,
    pub input_field_placeholder: Option<String>,
    pub selective: Option<bool>,
}
