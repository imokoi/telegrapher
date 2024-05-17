use serde::{Deserialize, Serialize};

use crate::models::location::Location;
use crate::models::user::User;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: User,
    pub location: Option<Location>,
    pub inline_message_id: Option<String>,
    pub query: String,
}
