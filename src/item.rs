use std::collections::HashMap;
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


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ItemTree {
    identifier: Identifier,
    parent: Option<Item>,
    childs: Vec<Box<ItemTree>>,
}

fn construct_tree<'a, 'b>(
    identifier: Identifier,
    parent: Option<Item>,
    item_map: &'b HashMap<Identifier, Item>,
) -> Result<ItemTree, String> {
    let item = item_map
        .get(&identifier)
        .ok_or(format!("Cannot find: {}", identifier))?;
    let mut item_tree = ItemTree {
        identifier,
        parent,
        childs: vec![],
    };
    for child_identifier in item.childs.iter() {
        let child_tree = construct_tree(
            child_identifier.clone(),
            Some(item.clone()),
            item_map
        )?;

        item_tree.childs.push(Box::new(child_tree));
    }

    Ok(item_tree)
}

impl ItemTree {
    pub fn new(
        root: Identifier,
        item_set: &HashMap<Identifier, Item>
    ) -> Result<Self, String> {
        construct_tree(root, None, item_set)
    }
}

impl Default for ItemTree {
    fn default() -> Self {
        ItemTree {
            identifier: Identifier::Reserved("## uninitilized".into()),
            childs: vec![],
            parent: None,
        }
    }
}
