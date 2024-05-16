use serde::{Deserialize, Serialize};

pub mod allowed_update;
pub mod animation;
pub mod audio;
pub mod boost;
pub mod business;
pub mod callback_query;
pub mod chat;
pub mod chat_member;
pub mod chosen_inline_result;
pub mod command;
pub mod contact;
pub mod dice;
pub mod document;
pub mod file;
pub mod forum;
pub mod game;
pub mod giveaway;
pub mod inline_query;
pub mod input_file;
pub mod input_media;
pub mod invoice;
pub mod link_preview;
pub mod location;
pub mod mask_position;
pub mod menu_button;
pub mod message;
pub mod message_entity;
pub mod order_info;
pub mod parse_mode;
pub mod passport_data;
pub mod passport_element_error;
pub mod photo_size;
pub mod poll;
pub mod pre_check_query;
pub mod proximity_alert_trigger;
pub mod reaction;
pub mod reply;
pub mod reply_markup;
pub mod shipping_query;
pub mod sticker;
pub mod sticker_set;
pub mod story;
pub mod successful_payment;
pub mod update;
pub mod user;
pub mod venue;
pub mod video;
pub mod video_chat;
pub mod video_note;
pub mod voice;
pub mod web_app;
pub mod webhook;

// #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
// #[serde(rename_all = "lowercase")]
// pub enum ChatType {
//     Private,
//     Group,
//     Supergroup,
//     Channel,
// }

// #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
// #[serde(rename_all = "snake_case")]
// pub enum MessageEntityType {
//     Mention,
//     Hashtag,
//     Cashtag,
//     BotCommand,
//     Url,
//     Email,
//     PhoneNumber,
//     Bold,
//     Italic,
//     Underline,
//     Strikethrough,
//     Spoiler,
//     Code,
//     Pre,
//     TextLink,
//     TextMention,
//     CustomEmoji,
//     Blockquote,
//     #[serde(other)]
//     Unknown,
// }

// #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
// #[serde(rename_all = "lowercase")]
// pub enum PollType {
//     Regular,
//     Quiz,
// }

// #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
// #[serde(rename_all = "snake_case")]
// pub enum PassportElementErrorDataFieldType {
//     PersonalDetails,
//     Passport,
//     DriverLicense,
//     IdentityCard,
//     InternalPassport,
//     Address,
// }

// #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
// #[serde(rename_all = "snake_case")]
// pub enum PassportElementErrorFrontSideType {
//     Passport,
//     DriverLicense,
//     IdentityCard,
//     InternalPassport,
// }

// #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
// #[serde(rename_all = "snake_case")]
// pub enum PassportElementErrorReverseSideType {
//     DriverLicense,
//     IdentityCard,
// }

// #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
// #[serde(rename_all = "snake_case")]
// pub enum PassportElementErrorSelfieType {
//     Passport,
//     DriverLicense,
//     IdentityCard,
//     InternalPassport,
// }

// #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
// #[serde(rename_all = "snake_case")]
// pub enum PassportElementErrorFileType {
//     UtilityBill,
//     BankStatement,
//     RentalAgreement,
//     PassportRegistration,
//     TemporaryRegistration,
// }

// #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
// #[serde(rename_all = "snake_case")]
// pub enum PassportElementErrorTranslationFileType {
//     Passport,
//     DriverLicense,
//     IdentityCard,
//     InternalPassport,
//     UtilityBill,
//     BankStatement,
//     RentalAgreement,
//     PassportRegistration,
//     TemporaryRegistration,
// }

// #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
// pub struct ResponseParameters {
//
//     pub migrate_to_chat_id: Option<i64>,

//
//     pub retry_after: Option<u16>,
// }
