use serde::{Deserialize, Serialize};

use crate::models::location::Location;
use crate::models::message_entity::MessageEntity;
use crate::models::parse_mode::ParseMode;
use crate::models::reply::InlineKeyboardMarkup;
use crate::models::user::User;

use super::link_preview::LinkPreviewOptions;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,
    pub location: Option<Location>,
    pub chat_type: Option<String>,
    pub query: String,
    pub offset: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultArticle {
    pub id: String,
    pub title: String,
    pub input_message_content: InputMessageContent,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub url: Option<String>,
    pub hide_url: Option<bool>,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_width: Option<u32>,
    pub thumbnail_height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultPhoto {
    pub id: String,
    pub photo_url: String,
    pub thumbnail_url: String,
    pub photo_width: Option<u32>,
    pub photo_height: Option<u32>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultGif {
    pub id: String,
    pub gif_url: String,
    pub gif_width: Option<u32>,
    pub gif_height: Option<u32>,
    pub gif_duration: Option<u32>,
    pub thumbnail_url: String,
    pub thumbnail_mime_type: Option<String>,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultMpeg4Gif {
    pub id: String,
    pub mpeg4_url: String,
    pub mpeg4_width: Option<u32>,
    pub mpeg4_height: Option<u32>,
    pub mpeg4_duration: Option<u32>,
    pub thumbnail_url: String,
    pub thumbnail_mime_type: Option<String>,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultVideo {
    pub id: String,
    pub video_url: String,
    pub mime_type: String,
    pub thumbnail_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub video_width: Option<u32>,
    pub video_height: Option<u32>,
    pub video_duration: Option<u32>,
    pub description: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultAudio {
    pub id: String,
    pub audio_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub performer: Option<String>,
    pub audio_duration: Option<u32>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultVoice {
    pub id: String,
    pub voice_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub voice_duration: Option<u32>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultDocument {
    pub id: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub document_url: String,
    pub mime_type: String,
    pub description: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_width: Option<u32>,
    pub thumbnail_height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultLocation {
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<u32>,
    pub heading: Option<u16>,
    pub proximity_alert_radius: Option<u32>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_width: Option<u32>,
    pub thumbnail_height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultVenue {
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    pub google_place_id: Option<String>,
    pub google_place_type: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_width: Option<u32>,
    pub thumbnail_height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultContact {
    pub id: String,
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub vcard: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_width: Option<u32>,
    pub thumbnail_height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InlineQueryResultGame {
    pub id: String,
    pub game_short_name: String,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultCachedPhoto {
    pub id: String,
    pub photo_file_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultCachedGif {
    pub id: String,
    pub gif_file_id: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultCachedMpeg4Gif {
    pub id: String,
    pub mpeg4_file_id: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultCachedSticker {
    pub id: String,
    pub sticker_file_id: String,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultCachedDocument {
    pub id: String,
    pub title: String,
    pub document_file_id: String,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultCachedVideo {
    pub id: String,
    pub video_file_id: String,
    pub title: String,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultCachedVoice {
    pub id: String,
    pub voice_file_id: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultCachedAudio {
    pub id: String,
    pub audio_file_id: String,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InputTextMessageContent {
    pub message_text: String,
    pub parse_mode: Option<ParseMode>,
    pub entities: Option<Vec<MessageEntity>>,
    pub link_preview_options: Option<LinkPreviewOptions>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct InputLocationMessageContent {
    pub latitude: f64,
    pub longitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<u32>,
    pub heading: Option<u16>,
    pub proximity_alert_radius: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputVenueMessageContent {
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    pub google_place_id: Option<String>,
    pub google_place_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InputContactMessageContent {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub vcard: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InputInvoiceMessageContent {
    pub title: String,
    pub description: String,
    pub payload: String,
    pub provider_token: String,
    pub currency: String,
    pub prices: Vec<LabeledPrice>,
    pub max_tip_amount: Option<u32>,
    pub suggested_tip_amounts: Option<Vec<u32>>,
    pub provider_data: Option<String>,
    pub photo_url: Option<String>,
    pub photo_size: Option<u32>,
    pub photo_width: Option<u32>,
    pub photo_height: Option<u32>,
    pub need_name: Option<bool>,
    pub need_phone_number: Option<bool>,
    pub need_email: Option<bool>,
    pub need_shipping_address: Option<bool>,
    pub send_phone_number_to_provider: Option<bool>,
    pub send_email_to_provider: Option<bool>,
    pub is_flexible: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum InputMessageContent {
    Text(InputTextMessageContent),
    Location(InputLocationMessageContent),
    Venue(InputVenueMessageContent),
    Contact(InputContactMessageContent),
    Invoice(InputInvoiceMessageContent),
}
