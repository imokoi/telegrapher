use serde::{Deserialize, Serialize};

use crate::models::user::User;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct VideoChatEnded {
    pub duration: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VideoChatParticipantsInvited {
    pub users: Option<Vec<User>>,
}
