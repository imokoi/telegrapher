use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<u32>,
    pub heading: Option<u16>,
    pub proximity_alert_radius: Option<u32>,
}
