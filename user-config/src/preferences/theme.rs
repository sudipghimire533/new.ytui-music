use serde::{Deserialize, Serialize};

use crate::styles::color::RGB;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Theme {
    pub base_color: RGB,
    pub highlight_color: RGB,
    pub active_color: RGB,
    pub inactive_color: RGB,
}
