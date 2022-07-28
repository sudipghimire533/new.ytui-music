use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(try_from = "&str")]
#[serde(into = "String")]
#[serde(deny_unknown_fields)]
pub enum Length {
    Absolute(u16),
    Relative(u16),
    AtLeast(u16),
    AtMost(u16),
    Fill,
}

impl From<Length> for String {
    fn from(length: Length) -> Self {
        match length {
            Length::Absolute(l) => format!("{l}a"),
            Length::Relative(l) => format!("{l}%"),
            Length::AtMost(l) => format!("{l}m"),
            Length::AtLeast(l) => format!("{l}l"),
            Length::Fill => "0f".to_string(),
        }
    }
}

impl TryFrom<&str> for Length {
    type Error = &'static str;

    fn try_from(source: &str) -> Result<Self, Self::Error> {
        let value_str = &source[..source.len() - 1];
        let unit = source.chars().last().ok_or("Invalid length string")?;

        let value = value_str.parse::<u16>().map_err(|_| "Invalid value")?;

        match unit {
            '%' => Ok(Length::Relative(value)),
            'a' => Ok(Length::Absolute(value)),
            'l' => Ok(Length::AtLeast(value)),
            'm' => Ok(Length::AtMost(value)),
            'f' => Ok(Length::Fill),
            _ => Err("Invalid unit"),
        }
    }
}

impl Length {
    pub fn get_absolute(&self, parent_length: u16, net_sibling_length: u16) -> u16 {
        let usable_length = parent_length
            .checked_sub(net_sibling_length)
            .expect("Siblings length must not have out grown parent length");
        match self {
            Length::Absolute(l) => usable_length.min(*l),
            Length::Relative(l) => usable_length.min(parent_length * l / 100),
            Length::AtLeast(l) => usable_length.max(*l),
            Length::AtMost(l) => usable_length.min(*l),
            Length::Fill => usable_length,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization_and_deserialization() {
        use Length::{Absolute, AtLeast, AtMost, Relative};
        let from_str = |s: &str| {
            let json_string = format!("\"{s}\"");
            serde_json::from_str(&json_string)
        };
        let to_string = |v: Length| {
            let json_string = serde_json::to_string(&v);
            json_string.map(|s| (&s[1..s.len() - 1]).to_string())
        };

        assert_eq!(Absolute(0), from_str("0a").unwrap());
        assert_eq!(Relative(10), from_str("10%").unwrap());
        assert_eq!(AtLeast(30), from_str("30l").unwrap());
        assert_eq!(AtMost(50), from_str("50m").unwrap());
        assert!(from_str(" 10u").is_err());
        assert!(from_str("10 u").is_err());
        assert!(from_str("100u ").is_err());
        assert!(from_str("u").is_err());
        assert!(from_str("100").is_err());
        assert!(from_str("100p").is_err());

        assert_eq!("10a", to_string(Absolute(10)).unwrap());
        assert_eq!("50%", to_string(Relative(50)).unwrap());
        assert_eq!("20m", to_string(AtMost(20)).unwrap());
        assert_eq!("30l", to_string(AtLeast(30)).unwrap());
    }
}
