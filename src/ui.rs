use crate::item::Grid;
use crate::popup::Popup;
use crate::window::Window;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct UI {
    window: Window,
    popup: Popup,
    grid: Grid,
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
            grid: vec![
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
