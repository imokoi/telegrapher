use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, Builder)]
#[builder(setter(into), default)]
pub struct LinkPreviewOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_small_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_large_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_above_text: Option<bool>,
}
