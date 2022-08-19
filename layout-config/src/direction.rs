use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(try_from = "String")]
#[serde(into = "&str")]
#[serde(deny_unknown_fields)]
pub enum Direction {
    Vertical,
    Horizontal,
}

// Convert Direction to static string
// Here it is intended to implement Into<> over From<>
// Since for reverse there is chance of failure
// so TryFrom is implemeneted later
#[allow(clippy::from_over_into)]
impl Into<&'static str> for Direction {
    fn into(self) -> &'static str {
        match self {
            Direction::Horizontal => "horizontal",
            Direction::Vertical => "vertical",
        }
    }
}

impl TryFrom<String> for Direction {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "vertical" => Ok(Direction::Vertical),
            "horizontal" => Ok(Direction::Horizontal),
            _ => Err("invalid direction"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization_and_deserialization() {
        use Direction::{Horizontal, Vertical};
        let from_str = |s: &str| {
            let json_string = format!("\"{s}\"");
            serde_json::from_str(&json_string)
        };
        let to_string = |v: Direction| {
            let json_string = serde_json::to_string(&v);
            json_string.map(|s| (&s[1..s.len() - 1]).to_string())
        };

        assert_eq!(Vertical, from_str("vertical").unwrap());
        assert_eq!(Horizontal, from_str("horizontal").unwrap());
        assert!(from_str("Horizontal").is_err());
        assert!(from_str("verTical").is_err());
        assert!(from_str(" vertical").is_err());

        assert_eq!("vertical", to_string(Vertical).unwrap());
        assert_eq!("horizontal", to_string(Horizontal).unwrap());
    }
}
