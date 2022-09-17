use crate::gadgets::state::AppState;
use crate::types::window::Window;
use user_config::action::KeyboardAction;
use user_config::action::KeyboardMapping;
use user_config::keyboard::Key;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum EventSummary {
    Execution(KeyboardAction),
    Resize,
    Nothing,
    Ignored,
}

pub fn listen_for_event(keyboard: &KeyboardMapping, appstate: &AppState) -> EventSummary {
    #[cfg(feature = "crossterm")]
    return crossterm_event::listen_for_event(keyboard, appstate);

    #[cfg(feature = "termion")]
    return termion_event::listen_for_event(keyboard);
}

pub fn handle_action(action: KeyboardAction, appstate: &mut AppState) {
    match action {
        KeyboardAction::Quit => (),
        KeyboardAction::ForceQuit => std::process::exit(1),
        KeyboardAction::PushSearchQuery(ch) => {
            appstate.altering_query.push(ch);
        }
        KeyboardAction::PopSearchQuery => {
            appstate.altering_query.pop();
        }

        KeyboardAction::GotoNextWindow => match appstate.active_window {
            Window::PaneWindow | Window::PaneTab => {
                if let Some(new_tab) = appstate.panetab_state.active_tab.next() {
                    (*appstate).panetab_state.active_tab = new_tab;
                } else {
                    (*appstate).active_window =
                        appstate.active_window.next().unwrap_or_else(Window::first);
                }
            }

            _ => {
                (*appstate).active_window =
                    appstate.active_window.next().unwrap_or_else(Window::first);
            }
        },

        KeyboardAction::GotoPrviousWindow => match appstate.active_window {
            Window::PaneWindow | Window::PaneTab => {
                if let Some(new_tab) = appstate.panetab_state.active_tab.prev() {
                    (*appstate).panetab_state.active_tab = new_tab;
                } else {
                    (*appstate).active_window =
                        appstate.active_window.prev().unwrap_or_else(Window::last)
                }
            }

            _ => {
                (*appstate).active_window =
                    appstate.active_window.prev().unwrap_or_else(Window::last)
            }
        },

        KeyboardAction::Escape => {
            (*appstate).active_window = appstate.active_window.next().unwrap_or_else(Window::first);
        }

        KeyboardAction::StartSearching => {
            (*appstate).active_window = crate::gadgets::window::Window::SearchBar;
        }
        KeyboardAction::Nothing => (),
        _ => todo!(),
    }
}

#[cfg(feature = "crossterm")]
mod crossterm_event {
    use crate::gadgets::window::Window;

    use super::*;
    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
    use std::time::Duration;

    // TODO:
    // make this configurable
    const REFRESH_RATE: Duration = Duration::from_secs(2);

    pub fn listen_for_event(keyboard: &KeyboardMapping, appstate: &AppState) -> EventSummary {
        if event::poll(REFRESH_RATE).map_err(|_| "crossterm pool event error") == Ok(true) {
            match event::read().unwrap() {
                Event::Resize(_col, _rows) => EventSummary::Resize,
                Event::Key(k) => {
                    let key = into_native_event(k);
                    let action = {
                        if appstate.active_window == Window::SearchBar {
                            if let Key::Char(ch) = key {
                                KeyboardAction::PushSearchQuery(ch)
                            } else if key == Key::Backspace {
                                KeyboardAction::PopSearchQuery
                            } else if key == Key::Esc {
                                KeyboardAction::GotoNextWindow
                            } else {
                                KeyboardAction::Nothing
                            }
                        } else {
                            keyboard.action_for(&key).unwrap_or(KeyboardAction::Nothing)
                        }
                    };

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

    pub fn listen_for_event(keyboard: &KeyboardMapping, app_state: &AppState) -> EventSummary {
        todo!()
    }
}
