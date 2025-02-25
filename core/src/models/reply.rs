use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::models::animation::Animation;
use crate::models::audio::Audio;
use crate::models::chat::Chat;
use crate::models::contact::Contact;
use crate::models::dice::Dice;
use crate::models::document::Document;
use crate::models::forum::CallbackGame;
use crate::models::game::Game;
use crate::models::giveaway::{Giveaway, GiveawayWinners};
use crate::models::invoice::Invoice;
use crate::models::link_preview::LinkPreviewOptions;
use crate::models::location::Location;
use crate::models::message_entity::MessageEntity;
use crate::models::photo_size::PhotoSize;
use crate::models::poll::Poll;
use crate::models::sticker::Sticker;
use crate::models::story::Story;
use crate::models::user::User;
use crate::models::venue::Venue;
use crate::models::video::Video;
use crate::models::video_note::VideoNote;
use crate::models::voice::Voice;
use crate::models::web_app::WebAppInfo;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TextQuote {
    pub text: String,
    pub entities: Option<Vec<MessageEntity>>,
    pub position: u64,
    pub is_manual: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ExternalReplyInfo {
    pub origin: Option<MessageOrigin>,
    pub chat: Option<Chat>,
    pub message_id: Option<i64>,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ReplyParameters {
    pub message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<TextQuote>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_position: Option<u64>,
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
    pub message_id: i64,
    pub author_signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginUrl {
    pub url: String,
    pub forward_text: Option<String>,
    pub bot_username: Option<String>,
    pub request_write_access: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
#[builder(setter(into))]
pub struct SwitchInlineQueryChosenChat {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_bot_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_group_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_channel_chats: Option<bool>,
}
