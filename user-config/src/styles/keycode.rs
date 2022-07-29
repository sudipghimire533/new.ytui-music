use serde::{Deserialize, Serialize};
use std::borrow::Cow;

fn combine_modifiers(modifier: &'static str, character: char) -> String {
    let mut res = String::with_capacity(modifier.len() + 1);
    res.push_str(modifier);
    res.push(character);
    res
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
struct KeyboardShortcut<'a>(Cow<'a, str>);

/// A key.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(into = "KeyboardShortcut")]
#[serde(try_from = "KeyboardShortcut")]
pub enum Key {
    /// Backspace.
    Backspace,
    /// Left arrow.
    Left,
    /// Right arrow.
    Right,
    /// Up arrow.
    Up,
    /// Down arrow.
    Down,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page Up key.
    PageUp,
    /// Page Down key.
    PageDown,
    /// Backward Tab key.
    BackTab,
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,
    /// TAB key
    Tab,
    /// Function keys.
    ///
    /// Only function keys 1 through 12 are supported.
    F(u8),
    /// Normal character.
    Char(char),
    /// Alt modified character.
    Alt(char),
    /// Ctrl modified character.
    ///
    /// Note that certain keys may not be modifiable with `ctrl`, due to limitations of terminals.
    Ctrl(char),
    /// Null byte.
    Null,
    /// Esc key.
    Esc,
}

impl From<Key> for Cow<'static, str> {
    fn from(key: Key) -> Self {
        match key {
            Key::Char(' ') => Cow::Borrowed("<space>"),
            Key::Char(c) => Cow::Owned(c.to_string()),
            Key::Ctrl(c) => Cow::Owned(combine_modifiers("<ctrl>", c)),
            Key::Esc => Cow::Borrowed("<esc>"),
            Key::PageUp => Cow::Borrowed("<pageUp>"),
            Key::PageDown => Cow::Borrowed("<pageDown>"),
            Key::Insert => Cow::Borrowed("<insert>"),
            Key::Backspace => Cow::Borrowed("<backspace>"),
            Key::Left => Cow::Borrowed("<left>"),
            Key::Right => Cow::Borrowed("<right>"),
            Key::Up => Cow::Borrowed("<up>"),
            Key::Down => Cow::Borrowed("<down>"),
            Key::Home => Cow::Borrowed("<home>"),
            Key::End => Cow::Borrowed("<end>"),
            Key::Tab => Cow::Borrowed("<tab>"),
            Key::Null => Cow::Borrowed("<null>"),
            Key::BackTab => Cow::Borrowed("<backtab>"),
            Key::Delete => Cow::Borrowed("<delete>"),
            Key::F(f) => Cow::Owned(format!("<F{f}>")),
            Key::Alt(c) => Cow::Owned(combine_modifiers("<alt>", c)),
        }
    }
}

impl TryFrom<Cow<'_, str>> for Key {
    type Error = &'static str;

    fn try_from(value: Cow<'_, str>) -> Result<Self, Self::Error> {
        let key = value.trim();

        if key.starts_with('<') {
            let mut chunks = key[1..].split('>');
            match chunks.next().ok_or("Invalid shortcut")? {
                "space" => Ok(Key::Char(' ')),
                "alt" => {
                    let inp = chunks
                        .next()
                        .ok_or("Alt followed by nothing")?
                        .chars()
                        .next()
                        .ok_or("Empty string for Alt")?;
                    Ok(Key::Alt(inp))
                }
                "ctrl" => {
                    let inp = chunks
                        .next()
                        .ok_or("Ctrl following by nothing")?
                        .chars()
                        .next()
                        .ok_or("Empty string for ctrl")?;
                    Ok(Key::Ctrl(inp))
                }
                "left" => Ok(Key::Left),
                "right" => Ok(Key::Right),
                "up" => Ok(Key::Up),
                "down" => Ok(Key::Down),
                "esc" => Ok(Key::Esc),
                "backspace" => Ok(Key::Backspace),
                "tab" => Ok(Key::Tab),
                "insert" => Ok(Key::Insert),
                "backtab" => Ok(Key::Backspace),
                "null" => Ok(Key::Null),
                "delete" => Ok(Key::Delete),
                "home" => Ok(Key::Home),
                "end" => Ok(Key::End),
                "f1" => Ok(Key::F(1)),
                "f2" => Ok(Key::F(2)),
                "f3" => Ok(Key::F(3)),
                "f4" => Ok(Key::F(4)),
                "f5" => Ok(Key::F(5)),
                "f6" => Ok(Key::F(6)),
                "f7" => Ok(Key::F(7)),
                "f8" => Ok(Key::F(8)),
                "f9" => Ok(Key::F(9)),
                "f10" => Ok(Key::F(10)),
                "f11" => Ok(Key::F(11)),
                "f12" => Ok(Key::F(12)),
                _ => Err("invalid shortcut"),
            }
        } else {
            let inp = key.chars().next().ok_or("Empty string as char")?;
            Ok(Key::Char(inp))
        }
    }
}

impl From<Key> for KeyboardShortcut<'_> {
    fn from(key: Key) -> Self {
        KeyboardShortcut(key.into())
    }
}

impl TryFrom<KeyboardShortcut<'_>> for Key {
    type Error = &'static str;

    fn try_from(value: KeyboardShortcut) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}
