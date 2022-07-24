use layout_config::size::Size;

pub trait ExtendSize {
    // Given the size of parent element
    // return what size can be applied to self
    fn get_appliable_size(&self, parent_length: u16) -> u16;
}

impl ExtendSize for Size {
    fn get_appliable_size(&self, parent_length: u16) -> u16 {
        use std::cmp::{min, max};

        let minimum_length = self.minimum.get_absolute(parent_length);
        let maximum_length = self.maximum.get_absolute(parent_length);
        let preferred_length = self.preferred.get_absolute(parent_length);

        min(
            min(maximum_length, parent_length),
            max(preferred_length, minimum_length),
        )
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use layout_config::length::Length;

    #[test]
    fn get_applicable_size() {
        let size = Size {
            preferred: Length::Relative(50),
            minimum: Length::Absolute(20),
            maximum: Length::Absolute(100),
        };
        assert_eq!(size.get_appliable_size(70), 35);
        assert_eq!(size.get_appliable_size(20), 20);
        assert_eq!(size.get_appliable_size(10), 10);
        assert_eq!(size.get_appliable_size(200), 100);
        assert_eq!(size.get_appliable_size(300), 100);

        let size = Size {
            preferred: Length::Absolute(20),
            minimum: Length::Relative(25),
            maximum: Length::Relative(50),
        };
        assert_eq!(size.get_appliable_size(100), 25);
        assert_eq!(size.get_appliable_size(20), 10);
        assert_eq!(size.get_appliable_size(4), 2);

        let size = Size {
            preferred: Length::Absolute(20),
            minimum: Length::Relative(50),
            maximum: Length::Absolute(50),
        };
        assert_eq!(size.get_appliable_size(100), 50);
        assert_eq!(size.get_appliable_size(40), 20);
        assert_eq!(size.get_appliable_size(4), 4);
        assert_eq!(size.get_appliable_size(17), 17);
    }

}
