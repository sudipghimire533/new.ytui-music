use crate::length::Length;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Size {
    pub minimum: Length,
    pub maximum: Length,
    pub preferred: Length,
}

impl Default for Size {
    fn default() -> Self {
        Size {
            minimum: Length::Absolute(0),
            maximum: Length::Relative(100),
            preferred: Length::Relative(100),
        }
    }
}
