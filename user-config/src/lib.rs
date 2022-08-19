use layout_config::ui::UI;
use preferences::theme::Theme;
use action::KeyboardAction;
use serde::{Deserialize, Serialize};

pub mod action;
pub mod keyboard;
pub mod preferences;
pub mod styles;

pub mod reexports {
    pub use layout::rect::Rect;
    pub use layout::rect_computation::compute_rect_for_item_tree;
    pub use layout_config::identifier::Identifier;
    pub use layout_config::item::ItemTree;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub layout: UI,
    pub theme: Theme,
    pub keyboard: KeyboardAction,
}
