pub mod color;
pub mod keycode;

pub struct Style {
    fg: Option<color::RGB>,
    bg: Option<color::RGB>,
    modifier: tui::style::Modifier,
}
