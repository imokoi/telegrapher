use crate::models::allowed_update::AllowedUpdate;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into), default)]
pub struct GetUpdatesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<AllowedUpdate>>,
}
