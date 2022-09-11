use user_config::action::KeyboardAction;
use user_config::action::KeyboardMapping;
use user_config::keyboard::Key;

pub fn listen_for_event(keyboard: &KeyboardMapping) -> EventSummary {
    #[cfg(feature = "crossterm")]
    return crossterm_event::listen_for_event(keyboard);

    #[cfg(feature = "termion")]
    return termion_event::listen_for_event(keyboard);
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum EventSummary {
    Execution(KeyboardAction),
    Resize,
    Nothing,
    Ignored,
}

#[cfg(feature = "crossterm")]
mod crossterm_event {
    use super::*;
    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
    use std::time::Duration;

    // TODO:
    // make this configurable
    const REFRESH_RATE: Duration = Duration::from_secs(2);

    pub fn listen_for_event(keyboard: &KeyboardMapping) -> EventSummary {
        if event::poll(REFRESH_RATE).map_err(|_| "crossterm pool event error") == Ok(true) {
            match event::read().unwrap() {
                Event::Resize(_col, _rows) => EventSummary::Resize,
                Event::Key(k) => {
                    let key = into_native_event(k);
                    let action = keyboard.action_for(&key);

                    EventSummary::Execution(action)
                }
                _ => EventSummary::Ignored,
            }
        } else {
            EventSummary::Nothing
        }
    }

    fn into_native_event(key: KeyEvent) -> Key {
        match key.code {
            KeyCode::BackTab => Key::BackTab,
            KeyCode::Backspace => Key::Backspace,
            KeyCode::Up => Key::Up,
            KeyCode::Down => Key::Down,
            KeyCode::PageDown => Key::PageDown,
            KeyCode::PageUp => Key::PageUp,
            KeyCode::Esc => Key::Esc,
            KeyCode::End => Key::End,
            KeyCode::Delete => Key::Delete,
            KeyCode::Left => Key::Left,
            KeyCode::Right => Key::Right,
            KeyCode::Null => Key::Null,
            KeyCode::Insert => Key::Insert,
            KeyCode::Home => Key::Home,
            KeyCode::Tab => Key::Tab,
            KeyCode::F(n) => Key::F(n),
            KeyCode::Char(c) => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    Key::Ctrl(c)
                } else if key.modifiers.contains(KeyModifiers::ALT) {
                    Key::Alt(c)
                } else {
                    Key::Char(c)
                }
            }
            _ => Key::Null,
        }
    }
}

#[cfg(feature = "termion")]
mod termion_event {
    use super::*;

    pub fn listen_for_event(keyboard: &KeyboardMapping) -> EventSummary {}
}
