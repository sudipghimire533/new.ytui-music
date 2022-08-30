use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt::Display};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "camelCase")]
#[serde(try_from = "String")]
#[serde(into = "String")]
#[serde(deny_unknown_fields)]
pub enum Identifier {
    Container(String),
    Gadget(Cow<'static, str>),
}

impl Identifier {
    pub fn is_container(&self) -> bool {
        matches!(self, Identifier::Container(..))
    }

    pub fn is_gadget(&self) -> bool {
        matches!(self, Identifier::Gadget(..))
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id_str = match self {
            Identifier::Gadget(r) => r,
            Identifier::Container(c) => c.as_str(),
        };
        write!(f, "{id_str}")
    }
}

impl<'a> From<&'a Identifier> for &'a str {
    fn from(identifier: &'a Identifier) -> Self {
        match identifier {
            Identifier::Container(id) => id,
            Identifier::Gadget(id) => id,
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
        let is_custom = src
            .chars()
            .next()
            .ok_or("Empty identifier is not valid")?
            .is_uppercase();

        if is_custom {
            Ok(Identifier::Container(src))
        } else {
            Ok(Identifier::Gadget(src.into()))
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
        let container_identifiers = ["TopArea", "Something", "Uppercase"];
        let gadget_identifiers = ["searchbar", "something", "lowercase", "_Started", "_starts"];

        for container in container_identifiers {
            assert_eq!(
                Ok(Identifier::Container(container.to_string())),
                container.try_into()
            );
        }

        for gadget in gadget_identifiers {
            assert_eq!(Ok(Identifier::Gadget(gadget.into())), gadget.try_into());
        }
    }

    #[test]
    fn serialization_and_deserialization() {
        use Identifier::{Container, Gadget};
        let from_str = |s: &str| {
            let json_string = format!("\"{s}\"");
            serde_json::from_str(&json_string)
        };
        let to_string = |v: Identifier| {
            let json_string = serde_json::to_string(&v);
            json_string.map(|s| (&s[1..s.len() - 1]).to_string())
        };

        assert_eq!(Gadget("cUstom".into()), from_str("cUstom").unwrap());
        assert_eq!(Gadget("a_name".into()), from_str("a_name").unwrap());
        for container in ["TopArea", "Constainer", "Apple"] {
            assert_eq!(
                Container(container.to_string()),
                from_str(container).unwrap()
            );

            assert_eq!(
                container.to_string(),
                to_string(Gadget(Cow::Borrowed(container))).unwrap()
            );
        }

        assert_eq!("cUstom", to_string(Container("cUstom".into())).unwrap());
        assert_eq!("my_Name", to_string(Container("my_Name".into())).unwrap());
        assert_eq!("cUst0m@", to_string(Container("cUst0m@".into())).unwrap());
    }
}
