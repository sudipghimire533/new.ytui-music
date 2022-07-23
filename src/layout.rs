use std::borrow::Cow;

use serde::{Serialize, Deserialize};

const RESERVED_IDENTIFIER: &[&str] = &[
    "Red_element",
    "Blue_element",
    "Green_element",
    "Yellow_element",
];

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(try_from = "&str")]
#[serde(into = "&str")]
#[serde(deny_unknown_fields)]
pub enum LayoutDirection {
    Vertical,
    Horizontal,
}

impl Into<&'static str> for LayoutDirection {
    fn into(self) -> &'static str {
        match self {
            LayoutDirection::Horizontal => "horizontal",
            LayoutDirection::Vertical => "vertical",
        }
    }
}

impl TryFrom<&str> for LayoutDirection {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "vertical" => Ok(LayoutDirection::Vertical),
            "horizontal" => Ok(LayoutDirection::Horizontal),
            _ => Err("invalid direction")
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(try_from = "String")]
#[serde(into = "String")]
#[serde(deny_unknown_fields)]
pub enum Identifier {
    Custom(String),
    Reserved(Cow<'static, str>),
}

impl<'a> From<&'a Identifier> for &'a str {
    fn from(identifier: &'a Identifier) -> Self {
        match identifier {
            Identifier::Custom(id) => &id,
            Identifier::Reserved(id) => &id,
        }
    }
}

impl From<Identifier> for String {
    fn from(identifier: Identifier) -> Self {
        <&Identifier as Into<&str>>::into(&identifier).to_string()
    }
}

impl TryFrom<String> for Identifier{
    type Error = &'static str;

    fn try_from(src: String) -> Result<Self, Self::Error> {
       match RESERVED_IDENTIFIER
            .iter()
            .find(|&&id| id == src)
        {
            // If one of rserved identifier just return it
            Some(reserved_id) => {
                Ok(
                    Identifier::Reserved(Cow::Borrowed(*reserved_id))
                )
            },

            // If not verify that it is valid identifier:
            // - should contains only ascii alphabet or _
            None if src.len() > 0 => src
                .chars()
                .all(|c| {
                    c.is_ascii_alphabetic() || c == '_'
                })
                .then_some(Identifier::Custom(src))
                .ok_or("Invalid identifier"),

            // If this is empty identifier
            _ => Err("Empty identifier is not valid")
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(try_from = "&str")]
#[serde(into = "String")]
#[serde(deny_unknown_fields)]
pub enum UnitLength {
    Absolute(u16),
    Relative(u16),
}

impl From<UnitLength> for String {
    fn from(length: UnitLength) -> Self {
        match length {
            UnitLength::Absolute(l) => format!("{l}u"),
            UnitLength::Relative(l) => format!("{l}%"),
        }
    }
}

impl TryFrom<&str> for UnitLength {
    type Error = &'static str;

    fn try_from(source: &str) -> Result<Self, Self::Error> {

        let value_str = &source[..source.len() - 1];
        let unit_str = source.chars().last().ok_or("Invalid length string")?;

        let value = value_str.parse::<u16>()
            .map_err(|_| "Invalid value for length")?;

        if unit_str == '%' {
            Ok(UnitLength::Relative(value))
        } else if unit_str == 'u' {
            Ok(UnitLength::Absolute(value))
        } else {
            Err("Length doesn't ends with valid unit")
        }

    }
}

impl UnitLength {
    fn get_absolute(&self, parent_length: u16) -> u16 {
        match self {
            UnitLength::Absolute(l) => *l,
            UnitLength::Relative(l) => parent_length * l / 100
        }
    }

    fn make_absolute(&mut self, parent_length: u16) -> u16 {
        let absolute_len = self.get_absolute(parent_length);
        (*self) = UnitLength::Absolute(absolute_len);
        
        absolute_len
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Size {
    minimum: UnitLength,
    maximum: UnitLength,
    preferred: UnitLength,
}

impl Default for Size {
    fn default() -> Self {
        Size {
            minimum: UnitLength::Absolute(0),
            maximum: UnitLength::Relative(100),
            preferred: UnitLength::Relative(100),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Item {
    identifier: Identifier,
    size: Size,
    childs: Vec<Identifier>,
    split: LayoutDirection,
}
type Grid = Vec<Item>;


#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Window {
    render: Identifier,
    height: Size,
    width: Size,
}


#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Popup {
    height: Size,
    width: Size,
}


#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct UiLayout {
    window: Window,
    popup: Popup,
    grid: Grid,
}

#[cfg(test)]
impl<'a> TryFrom<&'a str> for Identifier {
    type  Error = &'static str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        <Identifier as TryFrom<String>>::try_from(value.to_string())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_serde() {
        use UnitLength::{Absolute, Relative};
        let from_str = |s: &str| {
            let json_string = format!("\"{s}\"");
            serde_json::from_str(&json_string)
        };
        let to_string = |v: UnitLength|{
            let json_string = serde_json::to_string(&v);
            json_string.map(|s| {
                (&s[1..s.len()-1]).to_string()
            })
        };

        assert_eq!(Absolute(0), from_str("0u").unwrap());
        assert_eq!(Relative(10), from_str("10%").unwrap());
        assert!(from_str(" 10u").is_err());
        assert!(from_str("10 u").is_err());
        assert!(from_str("100u ").is_err());
        assert!(from_str("u").is_err());
        assert!(from_str("100").is_err());
        assert!(from_str("100p").is_err());

        assert_eq!("10u", to_string(Absolute(10)).unwrap());
        assert_eq!("50%", to_string(Relative(50)).unwrap());
    }

    #[test]
    fn identifier_serde() {
        use Identifier::{Reserved, Custom};
        let from_str = |s: &str| {
            let json_string = format!("\"{s}\"");
            serde_json::from_str(&json_string)
        };
        let to_string = |v: Identifier|{
            let json_string = serde_json::to_string(&v);
            json_string.map(|s| {
                (&s[1..s.len()-1]).to_string()
            })
        };

        assert_eq!(Custom("cUstom".into()), from_str("cUstom").unwrap());
        assert_eq!(Custom("a_name".into()), from_str("a_name").unwrap());
        assert!(from_str("an2me").is_err());
        assert!(from_str("m@ngo").is_err());

        for reserved in RESERVED_IDENTIFIER {
            assert_eq!(
                Reserved(Cow::Borrowed(reserved)),
                from_str(reserved).unwrap()
            );
        }
    }

    #[test]
    fn direction_serde() {
        use LayoutDirection::{Vertical, Horizontal};
        let from_str = |s: &str| {
            let json_string = format!("\"{s}\"");
            serde_json::from_str(&json_string)
        };
        let to_string = |v: LayoutDirection|{
            let json_string = serde_json::to_string(&v);
            json_string.map(|s| {
                (&s[1..s.len()-1]).to_string()
            })
        };

        assert_eq!(Vertical, from_str("vertical").unwrap());
        assert_eq!(Horizontal, from_str("horizontal").unwrap());
        assert!(from_str("Horizontal").is_err());
        assert!(from_str("verTical").is_err());
        assert!(from_str(" vertical").is_err());

        assert_eq!("vertical", to_string(Vertical).unwrap());
        assert_eq!("horizontal", to_string(Horizontal).unwrap());
    }

    #[test]
    fn window_serde() {
        use UnitLength::{Absolute, Relative};
        use Identifier::{Reserved, Custom};
        use serde_json::{from_str, to_string};

        let expected_window = Window {
            render: Custom("root".to_string()),
            height: Size {
                minimum: Absolute(300),
                maximum: Absolute(2000),
                preferred: Relative(100),
            },
            width: Size {
                minimum: Absolute(500),
                maximum: Absolute(1500),
                preferred: Relative(100),
            }
        };
        let expected_window_str = r##"{
                    "render":"root",
                    "height":{
                        "minimum": "300u",
                        "maximum": "2000u",
                        "preferred": "100%"
                    },
                    "width": {
                        "minimum": "500u",
                        "maximum": "1500u",
                        "preferred": "100%"
                    }
        }"##
        .replacen(" ", "", usize::MAX)
        .replacen("\r\n", "", usize::MAX)
        .replacen("\n", "", usize::MAX);

        assert_eq!(expected_window, from_str(&expected_window_str).unwrap());
        assert_eq!(
            expected_window_str,
            to_string(&expected_window).unwrap()
        );
    }

    #[test]
    fn popup_serde() {
         use UnitLength::{Absolute, Relative};
        use Identifier::{Reserved, Custom};
        use serde_json::{from_str, to_string};

        let expected_popup = Popup {
            height: Size {
                minimum: Relative(100),
                maximum: Relative(100),
                preferred: Relative(100),
            },
            width: Size {
                minimum: Relative(100),
                maximum: Relative(100),
                preferred: Relative(100),
            }
        };
        let expected_window_str = r##"{
                    "height":{
                        "minimum": "100%",
                        "maximum": "100%",
                        "preferred": "100%"
                    },
                    "width": {
                        "minimum": "100%",
                        "maximum": "100%",
                        "preferred": "100%"
                    }
        }"##
        .replacen(" ", "", usize::MAX)
        .replacen("\r\n", "", usize::MAX)
        .replacen("\n", "", usize::MAX);

        assert_eq!(expected_popup, from_str(&expected_window_str).unwrap());
        assert_eq!(
            expected_window_str,
            to_string(&expected_popup).unwrap()
        );
    }

    #[test]
    fn layout_file_valid() {
        let expected_layout_str = include_str!("../layout.json")
            .replacen(" ", "", usize::MAX)
            .replacen("\r\n", "", usize::MAX)
            .replacen("\n", "", usize::MAX);
        
        let expected_layout = UiLayout {
            window: Window {
                render: Identifier::Custom("root".into()),
                height: Size {
                    preferred: UnitLength::Relative(100),
                    minimum: UnitLength::Absolute(300),
                    maximum: UnitLength::Absolute(2000),
                },
                width: Size {
                    preferred: UnitLength::Relative(100),
                    minimum: UnitLength::Absolute(500),
                    maximum: UnitLength::Absolute(1500),
                }
            },
            popup: Popup {
                height: Size {
                    preferred: UnitLength::Relative(80),
                    maximum: UnitLength::Relative(80),
                    minimum: UnitLength::Relative(80)
                },
                width: Size {
                    preferred: UnitLength::Relative(80),
                    maximum: UnitLength::Relative(80),
                    minimum: UnitLength::Relative(80)
                }
            },
            grid: vec![
                Item {
                    identifier: "root".try_into().unwrap(),
                    size: Size {
                        preferred: UnitLength::Relative(100),
                        maximum: UnitLength::Relative(100),
                        minimum: UnitLength::Relative(100),
                    },
                    childs: vec![
                        "top_area".try_into().unwrap(),
                        "red_element_custom".try_into().unwrap(),
                    ],
                    split: LayoutDirection::Vertical,
                },
                Item {
                    identifier: "red_element_custom".try_into().unwrap(),
                    size: Size {
                        preferred: UnitLength::Absolute(5),
                        minimum: UnitLength::Absolute(5),
                        maximum: UnitLength::Absolute(5),
                    },
                    childs: vec!["Red_element".try_into().unwrap()],
                    split: LayoutDirection::Vertical,
                },
                Item {
                    identifier: "top_rea".try_into().unwrap(),
                    size: Size {
                        preferred: UnitLength::Relative(100),
                        minimum: UnitLength::Absolute(20),
                        maximum: UnitLength::Relative(100),
                    },
                    childs: vec![
                        "top_left".try_into().unwrap(),
                        "top_right".try_into().unwrap()
                    ],
                    split: LayoutDirection::Horizontal,
                },
                Item {
                    identifier: "top_left".try_into().unwrap(),
                    size: Size {
                        preferred: UnitLength::Relative(50),
                        maximum: UnitLength::Relative(50),
                        minimum: UnitLength::Relative(50),
                    },
                    childs: vec!["Blue_element".try_into().unwrap()],
                    split: LayoutDirection::Vertical
                },
                Item {
                    identifier: "top_right".try_into().unwrap(),
                    size: Size {
                        preferred: UnitLength::Relative(50),
                        maximum: UnitLength::Relative(50),
                        minimum: UnitLength::Relative(50),
                    },
                    childs: vec![
                        "green_container".try_into().unwrap(),
                        "yellow_container".try_into().unwrap(),
                        "blue_container".try_into().unwrap(),
                    ],
                    split: LayoutDirection::Vertical
                },
                Item {
                    identifier: "green_container".try_into().unwrap(),
                    size: Size {
                        preferred: UnitLength::Relative(33),
                        maximum: UnitLength::Relative(33),
                        minimum: UnitLength::Relative(33),
                    },
                    childs: vec!["Green_element".try_into().unwrap()],
                    split: LayoutDirection::Horizontal
                },
                Item {
                    identifier: "yellow_container".try_into().unwrap(),
                    size: Size {
                        preferred: UnitLength::Relative(33),
                        maximum: UnitLength::Relative(33),
                        minimum: UnitLength::Relative(33),
                    },
                    childs: vec!["Yellow_element".try_into().unwrap()],
                    split: LayoutDirection::Horizontal
                },
                Item {
                    identifier: "blue_container".try_into().unwrap(),
                    size: Size {
                        preferred: UnitLength::Relative(33),
                        maximum: UnitLength::Relative(33),
                        minimum: UnitLength::Relative(33),
                    },
                    childs: vec!["Blue_element".try_into().unwrap()],
                    split: LayoutDirection::Horizontal
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
