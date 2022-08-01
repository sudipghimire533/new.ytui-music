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

pub struct ShortcutPreferences {
    pub items: Vec<Shortcut>,
}
