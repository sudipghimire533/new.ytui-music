use crate::size::Size;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Window {
    pub height: Size,
    pub width: Size,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::length::Length;

    #[test]
    fn serialization_and_deserialization() {
        use serde_json::{from_str, to_string};

        let expected_window = Window {
            height: Length::Relative(100),
            width: Length::Relative(100),
        };
        let expected_window_str = r##"{
                    "height": "100%",
                    "width": "100%"
        }"##
        .replacen(' ', "", usize::MAX)
        .replacen("\r\n", "", usize::MAX)
        .replacen('\n', "", usize::MAX);

        assert_eq!(expected_window, from_str(&expected_window_str).unwrap());
        assert_eq!(expected_window_str, to_string(&expected_window).unwrap());
    }
}
