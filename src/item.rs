use crate::direction::Direction;
use crate::identifier::Identifier;
use crate::size::Size;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Item {
    pub identifier: Identifier,
    pub size: Size,
    pub childs: Vec<Identifier>,
    pub split: Direction,
}

pub type Grid = Vec<Item>;
