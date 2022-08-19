use action::KeyboardMapping;
use layout_config::length::Length;
use layout_config::ui::UI;
use preferences::theme::Theme;
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

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Config {
    pub layout: UI,
    pub theme: Theme,
    pub keyboard: KeyboardMapping,
}

pub fn default_config() -> Config {
    use action::KeyboardAction;
    use keyboard::Key;
    use layout_config::direction::Direction;
    use layout_config::identifier::Identifier::{Custom, Reserved};
    use layout_config::item::Item;
    use std::collections::HashMap;
    use styles::color::RGB;

    Config {
        layout: UI {
            window_height: Length::AtLeast(24),
            window_width: Length::AtLeast(80),
            popup_height: Length::Relative(80),
            popup_width: Length::Relative(80),
            item_root: [
                // root
                Item {
                    identifier: Custom("IAmRoot".to_string()),
                    size: Length::Relative(100),
                    childs: vec![
                        Custom("TopArea".to_string()),
                        Custom("MidArea".to_string()),
                        Custom("BotttomArea".to_string()),
                    ],
                    split: Direction::Vertical,
                },
                // -------------------
                // final gadgets
                // childs of these will be ignored so
                // `split` of these gadgets won't matter either
                //
                Item {
                    identifier: Reserved("searchbar".into()),
                    childs: [].to_vec(),
                    size: Length::Absolute(3),
                    split: Direction::Vertical,
                },
                Item {
                    identifier: Reserved("shortcuts".into()),
                    childs: [].to_vec(),
                    size: Length::Relative(30),
                    split: Direction::Vertical,
                },
                Item {
                    identifier: Reserved("panetab".into()),
                    childs: [].to_vec(),
                    size: Length::Absolute(3),
                    split: Direction::Vertical,
                },
                Item {
                    identifier: Reserved("result_pane".into()),
                    childs: [].to_vec(),
                    size: Length::Fill,
                    split: Direction::Vertical,
                },
                Item {
                    identifier: Reserved("gauge".into()),
                    childs: [].to_vec(),
                    size: Length::AtLeast(3),
                    split: Direction::Vertical,
                },
                //------

                // Containers to make the layout
                Item {
                    identifier: Custom("TopArea".to_string()),
                    childs: [Reserved("searchbar".into())].to_vec(),
                    split: Direction::Horizontal,
                    size: Length::Absolute(3),
                },
                Item {
                    identifier: Custom("MidArea".to_string()),
                    childs: [Reserved("shortcuts".into()), Custom("Central".to_string())].to_vec(),
                    size: Length::Relative(70),
                    split: Direction::Horizontal,
                },
                Item {
                    identifier: Custom("Central".to_string()),
                    childs: [Reserved("panetab".into()), Reserved("result_pane".into())].to_vec(),
                    size: Length::Fill,
                    split: Direction::Vertical,
                },
                Item {
                    identifier: Custom("BotttomArea".to_string()),
                    childs: [Reserved("gauge".into())].to_vec(),
                    size: Length::Absolute(3),
                    split: Direction::Vertical,
                },
            ]
            .to_vec()
            .try_into()
            .unwrap(),
        },
        theme: Theme {
            base_color: RGB(0, 0, 0),
            highlight_color: RGB(0, 0, 0),
            active_color: RGB(0, 0, 0),
            inactive_color: RGB(0, 0, 0),
        },
        keyboard: [
            (Key::Up, KeyboardAction::MoveUp),
            (Key::Down, KeyboardAction::MoveDown),
            (Key::Tab, KeyboardAction::GotoNextWindow),
            (Key::BackTab, KeyboardAction::GotoPrviousWindow),
            (Key::Right, KeyboardAction::SeekForward),
            (Key::Left, KeyboardAction::SeekBackward),
            (Key::Char(' '), KeyboardAction::PausePlay),
            (Key::Char('q'), KeyboardAction::Quit),
            (Key::Ctrl('c'), KeyboardAction::ForceQuit),
            (Key::Char('+'), KeyboardAction::VolumeUp),
            (Key::Char('-'), KeyboardAction::VolumeDown),
            (Key::Char('n'), KeyboardAction::NextTrack),
            (Key::Char('p'), KeyboardAction::PreviousTrack),
            (Key::Char('s'), KeyboardAction::ShuffleToggle),
            (Key::Char('r'), KeyboardAction::RepeatSwitch),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>()
        .into(),
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, fs::File, io::BufReader};
    use super::*;

    #[test]
    fn match_default_config() {
        let config_generated = default_config();
        let config_written: Config = {
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("res")
                .join("default-config.json");
            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap()
        };

        assert_eq!(config_written, config_generated);
    }
}
