use crate::{item::ItemTree, length::Length};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct UI {
    pub window_height: Length,
    pub window_width: Length,
    pub popup_height: Length,
    pub popup_width: Length,
    #[serde(rename = "items")]
    pub item_root: ItemTree,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::direction::Direction;
    use crate::item::Item;
    use crate::length::Length;

    #[test]
    fn layout_file_valid() {
        let expected_layout_str = include_str!("../layout.json")
            .replacen(' ', "", usize::MAX)
            .replacen("\r\n", "", usize::MAX)
            .replacen('\n', "", usize::MAX);

        let expected_layout = UI {
            window_width: Length::AtLeast(500),
            window_height: Length::AtLeast(300),
            popup_width: Length::Relative(80),
            popup_height: Length::Relative(80),
            item_root: vec![
                Item {
                    identifier: "Things_starts_from_me".try_into().unwrap(),
                    size: Length::Relative(100),
                    childs: vec![
                        "Red_element_custom".try_into().unwrap(),
                        "Bottom_area".try_into().unwrap(),
                    ],
                    split: Direction::Vertical,
                },
                Item {
                    identifier: "Red_element_custom".try_into().unwrap(),
                    size: Length::Absolute(5),
                    childs: vec!["red_element".try_into().unwrap()],
                    split: Direction::Vertical,
                },
                Item {
                    identifier: "Bottom_area".try_into().unwrap(),
                    size: Length::AtLeast(10),
                    childs: vec![
                        "Bottom_left".try_into().unwrap(),
                        "Bottom_right".try_into().unwrap(),
                    ],
                    split: Direction::Horizontal,
                },
                Item {
                    identifier: "Bottom_left".try_into().unwrap(),
                    size: Length::Relative(50),
                    childs: vec!["blue_element".try_into().unwrap()],
                    split: Direction::Vertical,
                },
                Item {
                    identifier: "Bottom_right".try_into().unwrap(),
                    size: Length::Relative(50),
                    childs: vec![
                        "Green_container".try_into().unwrap(),
                        "Yellow_container".try_into().unwrap(),
                        "Blue_container".try_into().unwrap(),
                    ],
                    split: Direction::Vertical,
                },
                Item {
                    identifier: "Green_container".try_into().unwrap(),
                    size: Length::Relative(33),
                    childs: vec!["green_element".try_into().unwrap()],
                    split: Direction::Horizontal,
                },
                Item {
                    identifier: "Yellow_container".try_into().unwrap(),
                    size: Length::Relative(33),
                    childs: vec!["yellow_element".try_into().unwrap()],
                    split: Direction::Horizontal,
                },
                Item {
                    identifier: "Blue_container".try_into().unwrap(),
                    size: Length::Fill,
                    childs: vec!["blue_element".try_into().unwrap()],
                    split: Direction::Horizontal,
                },
                // elements
                Item {
                    identifier: "blue_element".try_into().unwrap(),
                    size: Length::Fill,
                    childs: vec![],
                    split: Direction::Horizontal,
                },
                Item {
                    identifier: "yellow_element".try_into().unwrap(),
                    size: Length::Fill,
                    childs: vec![],
                    split: Direction::Horizontal,
                },
                Item {
                    identifier: "green_element".try_into().unwrap(),
                    size: Length::Fill,
                    childs: vec![],
                    split: Direction::Horizontal,
                },
                Item {
                    identifier: "red_element".try_into().unwrap(),
                    size: Length::Fill,
                    childs: vec![],
                    split: Direction::Horizontal,
                },
            ]
            .try_into()
            .unwrap(),
        };

        let actual_layout = serde_json::from_str::<UI>(&expected_layout_str).unwrap();
        assert_eq!(actual_layout.window_width, expected_layout.window_width);
        assert_eq!(actual_layout.window_height, expected_layout.window_height);
        assert_eq!(actual_layout.popup_width, expected_layout.popup_width);
        assert_eq!(actual_layout.popup_height, expected_layout.popup_height);
        assert_eq!(actual_layout.item_root, expected_layout.item_root);
        assert_eq!(actual_layout, expected_layout);
    }
}
