use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt::Display};

const RESERVED_IDENTIFIER: &[&str] = &[
    "Red_element",
    "Blue_element",
    "Green_element",
    "Yellow_element",
    "Root",
];

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "camelCase")]
#[serde(try_from = "String")]
#[serde(into = "String")]
#[serde(deny_unknown_fields)]
pub enum Identifier {
    Custom(String),
    Reserved(Cow<'static, str>),
}

impl Identifier {
    pub fn is_custom(&self) -> bool {
        match self {
            Identifier::Custom(..) => true,
            _ => false,
        }
    }

    pub fn is_reserved(&self) -> bool {
        !self.is_custom()
    }

    fn is_valid_identifier(identifier: &str) -> bool {
        identifier
            .chars()
            .all(|c| {
                c.is_ascii_alphabetic() || c == '_'
            })
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id_str = match self {
            Identifier::Reserved(r) => r,
            Identifier::Custom(c) => c.as_str(),
        };
        write!(f, "{}", id_str)
    }
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

impl TryFrom<String> for Identifier {
    type Error = &'static str;

    fn try_from(src: String) -> Result<Self, Self::Error> {
        match RESERVED_IDENTIFIER.iter().find(|&&id| id == src) {
            // If one of rserved identifier just return it
            Some(reserved_id) => Ok(Identifier::Reserved(Cow::Borrowed(*reserved_id))),

            // If not verify that it is valid identifier:
            // should contains only ascii alphabet or _
            None if src.len() > 0 => Identifier::is_valid_identifier(&src)
                .then_some(Identifier::Custom(src))
                .ok_or("Invalid identifier"),

            // If this is empty identifier
            _ => Err("Empty identifier is not valid"),
        }
    }
}

#[cfg(test)]
impl<'a> TryFrom<&'a str> for Identifier {
    type Error = &'static str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        <Identifier as TryFrom<String>>::try_from(value.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn identifier_set() {
        // Make sure all reserved identifier are valid
        for identifier in RESERVED_IDENTIFIER {
            assert_eq!(
                Ok(()),
                Identifier::is_valid_identifier(identifier)
                .then_some(())
                .ok_or(format!("{identifier} is not valid identifier"))
            );
        }
    }

    #[test]
    fn serialization_and_deserialization() {
        use Identifier::{Custom, Reserved};
        let from_str = |s: &str| {
            let json_string = format!("\"{s}\"");
            serde_json::from_str(&json_string)
        };
        let to_string = |v: Identifier| {
            let json_string = serde_json::to_string(&v);
            json_string.map(|s| (&s[1..s.len() - 1]).to_string())
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

            assert_eq!(
                reserved.to_string(),
                to_string(Reserved(Cow::Borrowed(reserved))).unwrap()
            );
        }

        assert_eq!("cUstom", to_string(Custom("cUstom".into())).unwrap());
        assert_eq!("my_Name", to_string(Custom("my_Name".into())).unwrap());
        assert_eq!("cUst0m@", to_string(Custom("cUst0m@".into())).unwrap());
    }
}
