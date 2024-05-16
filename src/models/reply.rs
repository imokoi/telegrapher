use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TextQuote {
    pub text: String,
    pub entities: Option<Vec<MessageEntity>>,
    pub position: u32,
    pub is_manual: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ExternalReplyInfo {
    pub origin: Option<MessageOrigin>,
    pub chat: Option<Chat>,
    pub message_id: Option<i32>,
    pub link_preview_options: Option<LinkPreviewOptions>,
    pub animation: Option<Animation>,
    pub audio: Option<Audio>,
    pub document: Option<Document>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub story: Option<Story>,
    pub video: Option<Video>,
    pub video_note: Option<VideoNote>,
    pub voice: Option<Voice>,
    pub has_media_spoiler: Option<bool>,
    pub contact: Option<Contact>,
    pub dice: Option<Dice>,
    pub game: Option<Game>,
    pub giveaway: Option<Giveaway>,
    pub giveaway_winners: Option<GiveawayWinners>,
    pub invoice: Option<Invoice>,
    pub location: Option<Location>,
    pub poll: Option<Poll>,
    pub venue: Option<Venue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplyParameters {
    pub message_id: i32,
    pub chat_id: Option<i32>,
    pub allow_sending_without_reply: Option<bool>,
    pub quote: Option<TextQuote>,
    pub quote_parse_mode: Option<String>,
    pub quote_entities: Option<Vec<MessageEntity>>,
    pub quote_position: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageOrigin {
    User(MessageOriginUser),
    HiddenUser(MessageOriginHiddenUser),
    Chat(MessageOriginChat),
    Channel(MessageOriginChannel),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageOriginUser {
    pub date: u64,
    pub sender_user: User,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageOriginHiddenUser {
    pub date: u64,
    pub sender_user_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageOriginChat {
    pub date: u64,
    pub sender_chat: Chat,
    pub author_signature: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageOriginChannel {
    pub date: u64,
    pub chat: Chat,
    pub message_id: i32,
    pub author_signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InlineKeyboardButton {
    pub text: String,
    pub url: Option<String>,
    pub login_url: Option<LoginUrl>,
    pub callback_data: Option<String>,
    pub web_app: Option<WebAppInfo>,
    pub switch_inline_query: Option<String>,
    pub switch_inline_query_current_chat: Option<String>,
    pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,
    pub callback_game: Option<CallbackGame>,
    pub pay: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginUrl {
    pub url: String,
    pub forward_text: Option<String>,
    pub bot_username: Option<String>,
    pub request_write_access: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SwitchInlineQueryChosenChat {
    pub query: Option<String>,
    pub allow_user_chats: Option<bool>,
    pub allow_bot_chats: Option<bool>,
    pub allow_group_chats: Option<bool>,
    pub allow_channel_chats: Option<bool>,
}
