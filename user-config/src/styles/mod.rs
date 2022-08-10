pub mod color;

pub struct Style {
    fg: Option<color::RGB>,
    bg: Option<color::RGB>,
    modifier: tui::style::Modifier,
}
