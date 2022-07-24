use crate::direction::Direction;
use crate::identifier::Identifier;
use crate::size::Size;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Item {
    pub identifier: Identifier,
    pub size: Size,
    pub childs: Vec<Identifier>,
    pub split: Direction,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(try_from = "serde_helper::ItemTreeAsVec")]
#[serde(into = "serde_helper::ItemTreeAsVec")]
pub struct ItemTree {
    pub item: Item,
    #[serde(skip)]
    pub parent: Option<Box<ItemTree>>,
    pub childs: Vec<Box<ItemTree>>,
}

fn print_tree(f: &mut std::fmt::Formatter, item: &ItemTree, mut indent: usize) -> std::fmt::Result {
    let name = &item.item.identifier;
    let previous_indent = std::iter::repeat("\u{205E}   ")
        .take(indent/4)
        .collect::<String>()
        .replacen("\u{205E}", " ", 1);
    let (new_line, self_indent) = if indent != 0 {
        ("\n", '\u{21B3}')
    } else {
        ("", '\u{229A}')
    };
    write!(f,"{new_line}{previous_indent}{self_indent} {name}")?;

    // repeat for childs too
    for child in item.childs.iter() {
        indent += 4;
        print_tree(f, child, indent)?;
        indent -= 4;
    }

    Ok(())
}

impl std::fmt::Display for ItemTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        print_tree(f, self, 0)
    }
}


mod serde_helper {
    use super::*;
    pub(super) type ItemTreeAsVec = Vec<Item>;

    fn construct_tree(
        item: Item,
        parent: Option<Box<ItemTree>>,
        item_map: &HashMap<Identifier, Item>,
    ) -> Result<ItemTree, String> {
        let mut item_tree = Box::new(ItemTree {
            item: item.clone(),
            parent,
            childs: vec![],
        });

        let child_containers = item.childs.iter().filter(|v| v.is_custom());

        for child_identifier in child_containers {
            let child_item = item_map
                .get(child_identifier)
                .ok_or(format!("Cannot find element {child_identifier}"))?;
            let child_tree = construct_tree(child_item.clone(), Some(item_tree.clone()), item_map)?;

            item_tree.childs.push(Box::new(child_tree));
        }

        Ok(*item_tree)
    }

    // Convert Item vector to tree
    // First item in the vector will be treated as super root
    // it's child will be read followed by randchild
    // until whole tree is being built
    // see comment on impl From<ItemTree> for ItemTreeAsVec
    // to see expected reverse conversion.
    // creation will be such that expectation of reverse conversion
    // holds true
    impl TryFrom<ItemTreeAsVec> for ItemTree {
        type Error = String;

        fn try_from(item_vec: ItemTreeAsVec) -> Result<Self, Self::Error> {
            let root_item = if let Some(root) = item_vec.first() {
                root.clone()
            } else {
                return Err("Empty item set found...".to_string());
            };

            let item_map = item_vec
                .into_iter()
                .map(|item| (item.identifier.clone(), item))
                .collect::<HashMap<Identifier, Item>>();

            construct_tree(root_item, None, &item_map)
                .map_err(|e| format!("Constructing tree from item set: {e:?}"))
        }
    }

    fn add_me_to_vec(tree: &ItemTree, target: &mut ItemTreeAsVec) {
        target.push(tree.item.clone());

        for child in tree.childs.iter() {
            add_me_to_vec(child, target)
        }
    }

    // Convert item tree back to vector
    // the order of vector should be deterministic
    // Current. Asume following tree
    //  - super_root
    // |____ - opt
    // |____ - usr
    // |_______ - hidden
    // |_______ - local
    // |___________ - bin
    // |___________ - scripts
    // |________- global
    // |____- var
    // In vector it will be:
    // [ super_root, opt, usr, hidden, local, bin, scripts, global, var ]
    //
    // So instead of inserting in map as we get from tree
    // we will first search for super root and then
    // proceed preserving order of child defined
    //
    // It should also be noted that
    // item with reserved id will not be on the list
    // so when encountered one, we won't read it's child neither itself
    impl From<ItemTree> for ItemTreeAsVec {
        fn from(item_tree: ItemTree) -> Self {
            let mut super_root = &item_tree;
            while let Some(parent) = &super_root.parent {
                super_root = parent;
            }

            let mut items = Vec::new();
            add_me_to_vec(super_root, &mut items);
            items
        }
    }
}