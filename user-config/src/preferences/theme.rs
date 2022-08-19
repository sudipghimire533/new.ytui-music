use crate::styles::color::RGB;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Theme {
    pub base_color: RGB,
    pub highlight_color: RGB,
    pub active_color: RGB,
    pub inactive_color: RGB,
}
