use crate::length::AbsoluteLength;
use crate::length::Length;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Size {
    pub minimum: Length,
    pub maximum: Length,
    pub preferred: Length,
}

use std::cmp::{max, min};
impl Size {
    // Given the size of parent element
    // return what size can be applied to self
    pub fn get_appliable_size(&self, parent_length: AbsoluteLength) -> AbsoluteLength {
        let minimum_length: u16 = self.minimum.get_absolute(parent_length).into();
        let maximum_length: u16 = self.maximum.get_absolute(parent_length).into();
        let preferred_length: u16 = self.preferred.get_absolute(parent_length).into();

        min(
            min(maximum_length, parent_length.into()),
            max(preferred_length, minimum_length),
        )
        .into()
    }
}

impl Default for Size {
    fn default() -> Self {
        Size {
            minimum: Length::Absolute(0),
            maximum: Length::Relative(100),
            preferred: Length::Relative(100),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_is_to_fill() {
        let minimum = Length::Absolute(0);
        let maximum = Length::Relative(100);
        let preferred = Length::Relative(100);

        let expected_default = Size {
            minimum,
            maximum,
            preferred,
        };

        assert_eq!(expected_default, Default::default());
    }

    #[test]
    fn get_applicable_size() {
        let size = Size {
            preferred: Length::Relative(50),
            minimum: Length::Absolute(20),
            maximum: Length::Absolute(100),
        };
        assert_eq!(size.get_appliable_size(70.into()), 35.into());
        assert_eq!(size.get_appliable_size(20.into()), 20.into());
        assert_eq!(size.get_appliable_size(10.into()), 10.into());
        assert_eq!(size.get_appliable_size(200.into()), 100.into());
        assert_eq!(size.get_appliable_size(300.into()), 100.into());

        let size = Size {
            preferred: Length::Absolute(20),
            minimum: Length::Relative(25),
            maximum: Length::Relative(50),
        };
        assert_eq!(size.get_appliable_size(100.into()), 25.into());
        assert_eq!(size.get_appliable_size(20.into()), 10.into());
        assert_eq!(size.get_appliable_size(4.into()), 2.into());

        let size = Size {
            preferred: Length::Absolute(20),
            minimum: Length::Relative(50),
            maximum: Length::Absolute(50),
        };
        assert_eq!(size.get_appliable_size(100.into()), 50.into());
        assert_eq!(size.get_appliable_size(40.into()), 20.into());
        assert_eq!(size.get_appliable_size(4.into()), 4.into());
        assert_eq!(size.get_appliable_size(17.into()), 17.into());
    }
}
