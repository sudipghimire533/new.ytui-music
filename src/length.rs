use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(try_from = "&str")]
#[serde(into = "String")]
#[serde(deny_unknown_fields)]
pub enum Length {
    Absolute(u16),
    Relative(u16),
}

impl From<Length> for String {
    fn from(length: Length) -> Self {
        match length {
            Length::Absolute(l) => format!("{l}u"),
            Length::Relative(l) => format!("{l}%"),
        }
    }
}

impl TryFrom<&str> for Length {
    type Error = &'static str;

    fn try_from(source: &str) -> Result<Self, Self::Error> {
        let value_str = &source[..source.len() - 1];
        let unit_str = source.chars().last().ok_or("Invalid length string")?;

        let value = value_str
            .parse::<u16>()
            .map_err(|_| "Invalid value for length")?;

        if unit_str == '%' {
            Ok(Length::Relative(value))
        } else if unit_str == 'u' {
            Ok(Length::Absolute(value))
        } else {
            Err("Length doesn't ends with valid unit")
        }
    }
}

impl Length {
    pub fn get_absolute(&self, parent_length: u16) -> u16 {
        match self {
            Length::Absolute(l) => *l,
            Length::Relative(l) => parent_length * l / 100,
        }
    }

    pub fn make_absolute(&mut self, parent_length: u16) -> u16 {
        let absolute_len = self.get_absolute(parent_length);
        (*self) = Length::Absolute(absolute_len);

        absolute_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization_and_deserialization() {
        use Length::{Absolute, Relative};
        let from_str = |s: &str| {
            let json_string = format!("\"{s}\"");
            serde_json::from_str(&json_string)
        };
        let to_string = |v: Length| {
            let json_string = serde_json::to_string(&v);
            json_string.map(|s| (&s[1..s.len() - 1]).to_string())
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
}
