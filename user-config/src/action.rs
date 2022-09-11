use crate::keyboard::Key;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[repr(transparent)]
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(transparent)]
pub struct KeyboardMapping(HashMap<Key, KeyboardAction>);

impl From<HashMap<Key, KeyboardAction>> for KeyboardMapping {
    fn from(map: HashMap<Key, KeyboardAction>) -> Self {
        KeyboardMapping(map)
    }
}

impl KeyboardMapping {
    pub fn new(mappings: HashMap<Key, KeyboardAction>) -> Self {
        KeyboardMapping(mappings)
    }

    pub fn action_for(&self, key: &Key) -> Option<KeyboardAction> {
        self.0.get(key).cloned()
    }
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
}
