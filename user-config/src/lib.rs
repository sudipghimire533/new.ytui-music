use layout_config::ui::UI;
use preferences::theme::Theme;
use serde::{Deserialize, Serialize};

pub mod keyboard;
pub mod preferences;
pub mod styles;

pub mod reexports {
    pub use layout_config::item::ItemTree;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub layout: UI,
    pub theme: Theme,
    pub keyboard: (),
}
