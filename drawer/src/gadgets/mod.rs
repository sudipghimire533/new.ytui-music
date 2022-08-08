pub mod searchbar;
use std::borrow::Cow;

use tui::widgets::ListState;
pub mod gauge;
pub mod panetab;
pub mod shortcut;
pub mod musicpane;

pub enum Query {
    SearchAll(String),
    SearchMusic(String),
    SearchPlaylist(String),
    Local,
    Trending,
    FollowingArtist,
    SavedPlaylist,
    LikedMusic,
}

pub struct FinalQuery(Cow<'static, str>);

impl Query {
    pub fn get_final_query(self) -> FinalQuery {
        match self {
            Query::SearchAll(mut search_query) => {
                search_query.push_str(":all");
                FinalQuery(search_query.into())
            }
            Query::SearchMusic(mut search_query) => {
                search_query.push_str(":music");
                FinalQuery(search_query.into())
            }
            Query::SearchPlaylist(mut search_query) => {
                search_query.push_str(":playlist");
                FinalQuery(search_query.into())
            }
            Query::Trending => FinalQuery(":trending".into()),
            Query::LikedMusic => FinalQuery(":liked_music".into()),
            Query::SavedPlaylist => FinalQuery(":saved_playlist".into()),
            Query::FollowingArtist => FinalQuery(":following_artist".into()),
            Query::Local => FinalQuery(":local".into()),
        }
    }
}

// TODO:
// use actual music unit type
pub struct MusicUnit;
pub struct PlaylistUnit;
pub struct ArtistUnit;

#[derive(PartialEq, Eq)]
pub enum Window {
    SearchBar,
    Shortcut,
    PaneTab,
    MusicPane,
    PlaylistPane,
    ArtistPane,
    None,
    Popup,
    Gauge,
}

pub struct QueryResult<T> {
    pub query: FinalQuery,
    pub list: Vec<T>,
}

/// This struct will contains data that
/// the app actually functions over
/// this include all the runtime mutable datas
pub struct AppState {
    /// Every result shown must be the outcome of some query
    /// For example,
    pub altering_query: String,
    pub music_result: QueryResult<MusicUnit>,
    pub artist_result: QueryResult<ArtistUnit>,
    pub playlist_result: QueryResult<PlaylistUnit>,
    pub active_window: Window,
    pub shortcut_list_state: ListState,
}
