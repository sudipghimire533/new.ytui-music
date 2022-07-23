use crate::item::Item;
use crate::item::ItemTree;
use crate::popup::Popup;
use crate::window::Window;
use crate::identifier::Identifier;
use std::collections::HashMap;
use std::borrow::Cow;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct UIBuilder {
    pub window: Option<Window>,
    pub popup: Option<Popup>,
    pub items: Vec<Item>,
}

impl UIBuilder {
    pub fn window(&mut self, window: Window) -> &mut Self {
        (*self).window = Some(window);
        self
    }

    pub fn popup(&mut self, popup: Popup) -> &mut Self {
        (*self).popup = Some(popup);
        self
    }

    pub fn extend_items(&mut self, items: Vec<Item>) -> &mut Self {
        (*self).items.extend(items);
        self
    }

    pub fn set_items(&mut self, items: Vec<Item>) -> &mut Self {
        (*self).items = items;
        self
    }

}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct UI {
    window: Window,
    popup: Popup,
    items: HashMap<Identifier, Item>,
    #[serde(skip)]
    item_tree: ItemTree,
}

impl UI {
    fn get_root_identifier(&self) -> &Identifier {
        &self.window.render
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn get_popup(&self) -> &Popup {
        &self.popup
    }

    pub fn get_items(&self) -> &HashMap<Identifier, Item> {
        &self.items
    }

    pub fn get_item_tree(&self) -> &ItemTree {
        &self.item_tree
    }

    pub fn set_items(
        &mut self,
        items: HashMap<Identifier, Item>
    ) -> Result<&mut Self, Cow<'static, str>> {
        (*self).item_tree = ItemTree::new(
                self.get_root_identifier().clone(),
                &items
            )
            .map_err(|e| {
                format!("While setting items in tree layout: {e:?}").to_owned()
            })?;
        (*self).items = items;

        Ok(self)
    }

    pub fn set_items_vec(
        &mut self,
        items: Vec<Item>
        ) -> Result<&mut Self, Cow<'static, str>> {
        let item_map = items
            .into_iter()
            .map(|v| {
                (v.identifier.clone(), v)
            })
            .collect();
        
        self.set_items(item_map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::direction::Direction;
    use crate::identifier::Identifier;
    use crate::item::Item;
    use crate::length::Length;
    use crate::size::Size;

    #[test]
    fn layout_file_valid() {
        let expected_layout_str = include_str!("../layout.json")
            .replacen(" ", "", usize::MAX)
            .replacen("\r\n", "", usize::MAX)
            .replacen("\n", "", usize::MAX);

        let expected_layout = UI {
            window: Window {
                render: Identifier::Custom("root".into()),
                height: Size {
                    preferred: Length::Relative(100),
                    minimum: Length::Absolute(300),
                    maximum: Length::Absolute(2000),
                },
                width: Size {
                    preferred: Length::Relative(100),
                    minimum: Length::Absolute(500),
                    maximum: Length::Absolute(1500),
                },
            },
            popup: Popup {
                height: Size {
                    preferred: Length::Relative(80),
                    maximum: Length::Relative(80),
                    minimum: Length::Relative(80),
                },
                width: Size {
                    preferred: Length::Relative(80),
                    maximum: Length::Relative(80),
                    minimum: Length::Relative(80),
                },
            },
            items: vec![
                Item {
                    identifier: "root".try_into().unwrap(),
                    size: Size {
                        preferred: Length::Relative(100),
                        maximum: Length::Relative(100),
                        minimum: Length::Relative(100),
                    },
                    childs: vec![
                        "top_area".try_into().unwrap(),
                        "red_element_custom".try_into().unwrap(),
                    ],
                    split: Direction::Vertical,
                },
                Item {
                    identifier: "red_element_custom".try_into().unwrap(),
                    size: Size {
                        preferred: Length::Absolute(5),
                        minimum: Length::Absolute(5),
                        maximum: Length::Absolute(5),
                    },
                    childs: vec!["Red_element".try_into().unwrap()],
                    split: Direction::Vertical,
                },
                Item {
                    identifier: "top_rea".try_into().unwrap(),
                    size: Size {
                        preferred: Length::Relative(100),
                        minimum: Length::Absolute(20),
                        maximum: Length::Relative(100),
                    },
                    childs: vec![
                        "top_left".try_into().unwrap(),
                        "top_right".try_into().unwrap(),
                    ],
                    split: Direction::Horizontal,
                },
                Item {
                    identifier: "top_left".try_into().unwrap(),
                    size: Size {
                        preferred: Length::Relative(50),
                        maximum: Length::Relative(50),
                        minimum: Length::Relative(50),
                    },
                    childs: vec!["Blue_element".try_into().unwrap()],
                    split: Direction::Vertical,
                },
                Item {
                    identifier: "top_right".try_into().unwrap(),
                    size: Size {
                        preferred: Length::Relative(50),
                        maximum: Length::Relative(50),
                        minimum: Length::Relative(50),
                    },
                    childs: vec![
                        "green_container".try_into().unwrap(),
                        "yellow_container".try_into().unwrap(),
                        "blue_container".try_into().unwrap(),
                    ],
                    split: Direction::Vertical,
                },
                Item {
                    identifier: "green_container".try_into().unwrap(),
                    size: Size {
                        preferred: Length::Relative(33),
                        maximum: Length::Relative(33),
                        minimum: Length::Relative(33),
                    },
                    childs: vec!["Green_element".try_into().unwrap()],
                    split: Direction::Horizontal,
                },
                Item {
                    identifier: "yellow_container".try_into().unwrap(),
                    size: Size {
                        preferred: Length::Relative(33),
                        maximum: Length::Relative(33),
                        minimum: Length::Relative(33),
                    },
                    childs: vec!["Yellow_element".try_into().unwrap()],
                    split: Direction::Horizontal,
                },
                Item {
                    identifier: "blue_container".try_into().unwrap(),
                    size: Size {
                        preferred: Length::Relative(33),
                        maximum: Length::Relative(33),
                        minimum: Length::Relative(33),
                    },
                    childs: vec!["Blue_element".try_into().unwrap()],
                    split: Direction::Horizontal,
                },
            ],
            item_tree: Default::default(),
        };

        assert_eq!(
            expected_layout_str,
            serde_json::to_string(&expected_layout).unwrap()
        );
        assert_eq!(
            expected_layout,
            serde_json::from_str(&expected_layout_str).unwrap()
        );
    }
}
