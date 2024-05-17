use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MaskPosition {
    pub point: String,
    pub x_shift: f64,
    pub y_shift: f64,
    pub scale: f64,
}
