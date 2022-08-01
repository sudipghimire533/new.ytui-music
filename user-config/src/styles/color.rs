use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use tui::style::Color;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
struct ColorCode<'a>(Cow<'a, str>);

impl From<RGB> for Color {
    fn from(rgb: RGB) -> Self {
        let RGB(r, g, b) = rgb;
        Color::Rgb(r, g, b)
    }
}

impl TryFrom<ColorCode<'_>> for RGB {
    type Error = &'static str;

    fn try_from(value: ColorCode<'_>) -> Result<Self, Self::Error> {
        let hex = value.0.trim_start_matches('#');

        let r = u8::from_str_radix(&hex[..2], 16).map_err(|_| "Invalid R value")?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid G value")?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid B value")?;

        Ok(RGB(r, g, b))
    }
}

impl From<RGB> for ColorCode<'_> {
    fn from(rgb: RGB) -> Self {
        let RGB(r, g, b) = rgb;
        ColorCode(format!("#{r:02X}{g:02X}{b:02X}").into())
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "ColorCode")]
#[serde(into = "ColorCode")]
pub struct RGB(pub u8, pub u8, pub u8);

impl RGB {
    pub fn get_rgb_tuple(self) -> (u8, u8, u8) {
        let RGB(r, g, b) = self;
        (r, g, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_to_color_code() {
        assert_eq!(ColorCode("#000000".into()), RGB(0, 0, 0).into());
        assert_eq!(ColorCode("#7B56C8".into()), RGB(123, 86, 200).into());
        assert_eq!(ColorCode("#FFFFFF".into()), RGB(255, 255, 255).into());
    }

    #[test]
    fn color_code_to_rgb() {
        assert_eq!(Ok(RGB(0, 0, 0)), ColorCode("#000000".into()).try_into());
        assert_eq!(
            Ok(RGB(123, 86, 200)),
            ColorCode("#7B56C8".into()).try_into()
        );
        assert_eq!(
            Ok(RGB(255, 255, 255)),
            ColorCode("#FFFFFF".into()).try_into()
        );
    }
}
