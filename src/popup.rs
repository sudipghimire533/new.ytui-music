use crate::size::Size;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Popup {
    pub height: Size,
    pub width: Size,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::length::Length;

    #[test]
    fn popup_serde() {
        use serde_json::{from_str, to_string};
        use Length::Relative;

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
            },
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
        assert_eq!(expected_window_str, to_string(&expected_popup).unwrap());
    }
}
