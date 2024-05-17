use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LinkPreviewOptions {
    pub is_disabled: Option<bool>,
    pub url: Option<String>,
    pub prefer_small_media: Option<bool>,
    pub prefer_large_media: Option<bool>,
    pub show_above_text: Option<bool>,
}
