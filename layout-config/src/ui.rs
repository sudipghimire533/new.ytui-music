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
                    identifier: "things_starts_from_me".try_into().unwrap(),
                    size: Length::Relative(100),
                    childs: vec![
                        "red_element_custom".try_into().unwrap(),
                        "bottom_area".try_into().unwrap(),
                    ],
                    split: Direction::Vertical,
                },
                Item {
                    identifier: "red_element_custom".try_into().unwrap(),
                    size: Length::Absolute(5),
                    childs: vec!["Red_element".try_into().unwrap()],
                    split: Direction::Vertical,
                },
                Item {
                    identifier: "bottom_area".try_into().unwrap(),
                    size: Length::AtLeast(10),
                    childs: vec![
                        "bottom_left".try_into().unwrap(),
                        "bottom_right".try_into().unwrap(),
                    ],
                    split: Direction::Horizontal,
                },
                Item {
                    identifier: "bottom_left".try_into().unwrap(),
                    size: Length::Relative(50),
                    childs: vec!["Blue_element".try_into().unwrap()],
                    split: Direction::Vertical,
                },
                Item {
                    identifier: "bottom_right".try_into().unwrap(),
                    size: Length::Relative(50),
                    childs: vec![
                        "green_container".try_into().unwrap(),
                        "yellow_container".try_into().unwrap(),
                        "blue_container".try_into().unwrap(),
                    ],
                    split: Direction::Vertical,
                },
                Item {
                    identifier: "green_container".try_into().unwrap(),
                    size: Length::Relative(33),
                    childs: vec!["Green_element".try_into().unwrap()],
                    split: Direction::Horizontal,
                },
                Item {
                    identifier: "yellow_container".try_into().unwrap(),
                    size: Length::Relative(33),
                    childs: vec!["Yellow_element".try_into().unwrap()],
                    split: Direction::Horizontal,
                },
                Item {
                    identifier: "blue_container".try_into().unwrap(),
                    size: Length::Fill,
                    childs: vec!["Blue_element".try_into().unwrap()],
                    split: Direction::Horizontal,
                },
            ]
            .try_into()
            .unwrap(),
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
