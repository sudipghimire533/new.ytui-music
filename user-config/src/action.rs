use crate::keyboard::Key;
use layout_config::window::Window;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap};

#[repr(transparent)]
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(transparent)]
pub struct KeyboardMapping(HashMap<MappingIndex, KeyboardAction>);

// In JSON representation this should be represented as:
// Examples:
// (Down, Some(MusicList))->MoveDown <=> "Down|MusicList": "MoveDown"
// (Up, Some(MusicList))->MoveUp <=> "Up|MusicList": "MoveUp"
// (Up, None)->PreviousWindow <=> "Up": "PreviousWindow"
//
// Window name is to be seperated by single `|`
// if nothing is behind last `|` or no `|` then take Option to be None
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct MappingIndex(Key, Option<Window>);

impl TryFrom<String> for MappingIndex {
    type Error = String;

    fn try_from(mapping_index: String) -> Result<Self, Self::Error> {
        if let Some((key_str, window_str)) = mapping_index.rsplit_once('|') {
            let key = serde_json::from_str::<Key>(key_str)
                .map_err(|_| format!("Invalid key in mapping index: {mapping_index}"))?;
            let window = serde_json::from_str::<Window>(window_str)
                .map_err(|_| format!("Invalid window in mapping index: {mapping_index}"))?;

            Ok(MappingIndex(key, Some(window)))
        } else {
            let key: Key = Cow::<'static, str>::Owned(mapping_index.clone())
                .try_into()
                .map_err(|e| format!("Invalid mapping index: {mapping_index}. Error: {e:?}"))?;

            Ok(MappingIndex(key, None))
        }
    }
}

impl From<MappingIndex> for String {
    fn from(MappingIndex(key, window): MappingIndex) -> Self {
        let key_str: std::borrow::Cow<'static, str> = key.into();
        let window_str = match window {
            None => String::new(),
            Some(window) => format!("|{window}"),
        };

        format!("{key_str}{window_str}")
    }
}

impl From<(Key, Option<layout_config::window::Window>)> for MappingIndex {
    fn from((key, window): (Key, Option<layout_config::window::Window>)) -> Self {
        MappingIndex(key, window)
    }
}

impl From<HashMap<MappingIndex, KeyboardAction>> for KeyboardMapping {
    fn from(map: HashMap<MappingIndex, KeyboardAction>) -> Self {
        KeyboardMapping(map)
    }
}

impl KeyboardMapping {
    pub fn new(mappings: HashMap<MappingIndex, KeyboardAction>) -> Self {
        KeyboardMapping(mappings)
    }

    pub fn action_for(&self, key: &MappingIndex) -> Option<KeyboardAction> {
        self.0.get(key).cloned()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum MoveDirection {
    Up,
    Left,
    Down,
    Right,
}

/// Possible set of actions that can be performed from keyboard
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum KeyboardAction {
    // Goto the searchbar
    StartSearching,
    // toggle stop/resume of playing track
    PausePlay,
    // Move down to the list
    MoveDown,
    // Move up to the list
    MoveUp,
    // go to next window
    GotoNextWindow,
    // go to previous window
    GotoPrviousWindow,
    // Select/ Enter/ Submit
    Execute,
    // Toggle shuffle on/off
    ShuffleToggle,
    // Toggle repeat one/ repeat playleat/ stop after current i.e no repeat
    RepeatSwitch,
    // bring volume down
    VolumeUp,
    // bring volume up
    VolumeDown,
    // expand details of item
    Expand,
    // Close any popup/ get out of insert mode in searchbar
    Escape,
    // Gracefully exit the application
    Quit,
    // Force quit application
    // usually std::process:quit
    ForceQuit,
    // Play next track
    NextTrack,
    // play previous track
    PreviousTrack,
    // Clear the results of current query
    ClearResult,
    // Show details of currenlty hilighted item
    ShowDetails,
    // about this binary
    // and user config
    AppDetails,
    // A action but really isn't
    Nothing,
    // Seek current track in forward direction
    SeekForward,
    // Seek current track in backward direction
    SeekBackward,
    // Insert this character to search query
    PushSearchQuery(char),
    // Remove last character from search query
    PopSearchQuery,
    // Move in PaneWindow
    MoveInPaneWindow(MoveDirection),
    // Move in Shortcut list
    MoveInShortcuts(MoveDirection),
    // Move in Music tab list
    MoveInMusicList(MoveDirection),
    // Move in Playlist tab list
    MoveInPlaylistList(MoveDirection),
    // Move in Artist tab list
    MoveInArtistList(MoveDirection),
}
