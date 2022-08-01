#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Shortcut {
    Trending,
    YoutubeCommunity,
    LikedSongs,
    MyPlaylist,
    FollowingArtist,
    Downloaded,
    Local,
    Search,
}

impl From<Shortcut> for &'static str {
    fn from(shortcut: Shortcut) -> Self {
        match shortcut {
            Shortcut::Trending => "Trending",
            Shortcut::Local => "Local",
            Shortcut::Search => "Search",
            Shortcut::LikedSongs => "Liked Songs",
            Shortcut::FollowingArtist => "Following",
            Shortcut::Downloaded => "Downloads",
            Shortcut::MyPlaylist => "Saved playlist",
            Shortcut::YoutubeCommunity => "Youtube Community",
        }
    }
}

pub struct ShortcutPreferences {
    pub items: Vec<Shortcut>,
}
