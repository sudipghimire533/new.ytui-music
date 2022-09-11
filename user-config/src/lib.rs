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
    use layout_config::identifier::Identifier::{Container, Gadget};
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
                    identifier: Container("IAmRoot".to_string()),
                    size: Length::Relative(100),
                    childs: vec![
                        Container("TopArea".to_string()),
                        Container("MidArea".to_string()),
                        Container("BotttomArea".to_string()),
                    ],
                    split: Direction::Vertical,
                },
                // -------------------
                // final gadgets
                // childs of these will be ignored so
                // `split` of these gadgets won't matter either
                //
                Item {
                    identifier: Gadget("searchbar".into()),
                    childs: [].to_vec(),
                    size: Length::Fill,
                    split: Direction::Vertical,
                },
                Item {
                    identifier: Gadget("shortcuts".into()),
                    childs: [].to_vec(),
                    size: Length::Relative(30),
                    split: Direction::Vertical,
                },
                Item {
                    identifier: Gadget("panetab".into()),
                    childs: [].to_vec(),
                    size: Length::Absolute(3),
                    split: Direction::Vertical,
                },
                Item {
                    identifier: Gadget("result_pane".into()),
                    childs: [].to_vec(),
                    size: Length::Fill,
                    split: Direction::Vertical,
                },
                Item {
                    identifier: Gadget("gauge".into()),
                    childs: [].to_vec(),
                    size: Length::Absolute(3),
                    split: Direction::Vertical,
                },
                //------

                // Containers to make the layout
                Item {
                    identifier: Container("TopArea".to_string()),
                    childs: [Gadget("searchbar".into())].to_vec(),
                    split: Direction::Horizontal,
                    size: Length::Absolute(3),
                },
                Item {
                    identifier: Container("MidArea".to_string()),
                    childs: [Gadget("shortcuts".into()), Container("Central".to_string())].to_vec(),
                    size: Length::Relative(70),
                    split: Direction::Horizontal,
                },
                Item {
                    identifier: Container("Central".to_string()),
                    childs: [Gadget("panetab".into()), Gadget("result_pane".into())].to_vec(),
                    size: Length::Fill,
                    split: Direction::Vertical,
                },
                Item {
                    identifier: Container("BotttomArea".to_string()),
                    childs: [Gadget("gauge".into())].to_vec(),
                    size: Length::AtLeast(3),
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
    use super::*;
    use std::{fs::File, io::BufReader, path::PathBuf};

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

        assert_eq!(config_written.keyboard, config_generated.keyboard);
        assert_eq!(config_written.theme, config_generated.theme);
        assert_eq!(config_written.layout, config_generated.layout);
        assert_eq!(config_written, config_generated);
    }
}
