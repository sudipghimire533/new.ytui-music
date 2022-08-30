use crate::direction::Direction;
use crate::identifier::Identifier;
use crate::length::Length;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Item {
    pub identifier: Identifier,
    pub size: Length,
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

    if item.item.identifier.is_gadget() {
        write!(f, " => {name}")?;
    } else {
        let previous_indent = "\u{205E}   "
            .repeat(indent / 4)
            .replacen('\u{205E}', " ", 1);
        let (new_line, self_indent) = if indent != 0 {
            ("\n", '\u{21B3}')
        } else {
            ("", '\u{229A}')
        };
        write!(f, "{new_line}{previous_indent}{self_indent} {name}")?;
    }

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
        item_map: &mut HashMap<Identifier, Item>,
    ) -> Result<ItemTree, String> {
        let mut item_tree = Box::new(ItemTree {
            item: item.clone(),
            parent,
            childs: vec![],
        });

        for child_identifier in item.childs.iter() {
            let child_tree;
            if child_identifier.is_container() {
                let child_item = item_map
                    .get(child_identifier)
                    .ok_or(format!("Cannot find element {child_identifier}"))?;
                child_tree = construct_tree(child_item.clone(), Some(item_tree.clone()), item_map)?;
            } else if child_identifier.is_gadget() {
                let parent_name = item.identifier.to_string();
                // There might be multiple final gadget under different containers
                // so by inclufing the name of container (parent) as well
                // we avoid clashing
                let final_child_id =
                    Identifier::Gadget(format!("{parent_name}->{child_identifier}").into());
                // first try to get specific item defined under this parent
                // if not, use the global element of child_identifier
                let mut child_item = item_map
                    .get(&final_child_id)
                    .map(|v| Some(v))
                    .unwrap_or_else(|| item_map.get(&child_identifier))
                    .cloned()
                    .ok_or(format!("Required item not defined. One of `{final_child_id}` or `{child_identifier}` must be defined"))?;
                child_item.identifier = final_child_id;
                child_tree = ItemTree {
                    item: child_item.clone(),
                    childs: vec![],
                    parent: Some(Box::new(item_tree.as_ref().clone())),
                };
            } else {
                unreachable!("Identifier is either reserved or custom")
            }

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

            let mut item_map = item_vec
                .into_iter()
                .map(|item| (item.identifier.clone(), item))
                .collect::<HashMap<Identifier, Item>>();

            construct_tree(root_item, None, &mut item_map)
                .map_err(|e| format!("Constructing tree from item set: {e:?}"))
        }
    }

    fn add_me_to_vec(tree: &ItemTree, target: &mut ItemTreeAsVec) {
        target.push(tree.item.clone());

        for child in tree.childs.iter() {
            //.filter(|c| c.item.identifier.is_container()) {
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

#[cfg(test)]
mod tests {
    use super::*;

    // Only possibility of having multiple final element is to keep them in two
    // different container
    // duplicate element in same container is not supported
    #[test]
    fn duplicate_element() {
        let tree_as_vec = vec![
            Item {
                identifier: Identifier::Container("Root".to_string()),
                childs: vec![
                    Identifier::Container("Container".to_string()),
                    Identifier::Gadget("element".into()),
                ],
                split: Direction::Vertical,
                size: Length::Fill,
            },
            Item {
                identifier: Identifier::Container("Container".to_string()),
                childs: vec![Identifier::Gadget("element".into())],
                split: Direction::Vertical,
                size: Length::Fill,
            },
            Item {
                identifier: Identifier::Gadget("element".into()),
                childs: vec![],
                split: Direction::Vertical,
                size: Length::Absolute(10),
            },
        ];

        let root: ItemTree = tree_as_vec.clone().try_into().unwrap();
        let container = root.childs[0].clone();
        let first_element = root.childs[1].clone();
        let second_element = container.childs[0].clone();

        assert_eq!(root.childs, &[container.clone(), first_element.clone()]);
        assert_eq!(first_element.childs, &[]);
        assert_eq!(second_element.childs, &[]);
    }
}
