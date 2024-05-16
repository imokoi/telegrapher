use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StickerSet {
    pub name: String,
    pub title: String,
    #[serde(rename = "sticker_type")]
    pub sticker_type: StickerType,

    #[doc(hidden)]
    #[deprecated(since = "0.19.2", note = "Please use `sticker_type` instead")]
    pub contains_masks: bool,
    pub stickers: Vec<Sticker>,
    pub thumbnail: Option<PhotoSize>,
}
