use crate::models::{allowed_update::AllowedUpdate, sticker::InputFile};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into), default)]
pub struct SetWebhookParams {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<AllowedUpdate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_token: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into), default)]
pub struct DeleteWebhookParams {
    pub drop_pending_updates: bool,
}
