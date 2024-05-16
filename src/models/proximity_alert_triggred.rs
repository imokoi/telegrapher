use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProximityAlertTriggered {
    pub traveler: User,
    pub watcher: User,
    pub distance: u32,
}
