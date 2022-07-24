#[derive(Debug, Clone, Default)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub height: u16,
    pub width: u16,
}

impl Rect {
    pub fn area(&self) -> u16 {
        self.height * self.width
    }
}
