use serde::{Deserialize, Serialize};

use crate::models::file::File;
use crate::models::mask_position::MaskPosition;
use crate::models::photo_size::PhotoSize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sticker {
    pub file_id: String,
    pub file_unique_id: String,
    #[serde(rename = "type")]
    pub sticker_type: String,
    pub width: u32,
    pub height: u32,
    pub is_animated: bool,
    pub is_video: bool,
    pub thumbnail: Option<PhotoSize>,
    pub emoji: Option<String>,
    pub set_name: Option<String>,
    pub premium_animation: Option<File>,
    pub mask_position: Option<MaskPosition>,
    pub custom_emoji_id: Option<String>,
    pub needs_repainting: Option<bool>,
    pub file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StickerType {
    Regular,
    Mask,
    CustomEmoji,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputSticker {
    pub sticker: FileUpload,
    pub format: StickerFormat,
    pub emoji_list: Vec<String>,
    pub mask_position: Option<MaskPosition>,
    pub keywords: Option<Vec<String>>,
}
