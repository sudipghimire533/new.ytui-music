pub mod searchbar;
use std::borrow::Cow;

use tui::{widgets::{ListState, TableState}, layout::{Rect, Constraint}};
pub mod gauge;
pub mod musicpane;
pub mod playlistpane;
pub mod panetab;
pub mod shortcut;

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

pub struct MusicUnit {
    pub title: String,
    pub artist: String,
    pub duration: String, // TODO:
}

pub struct PlaylistUnit {
    pub title: String,
    pub creator: String,
    pub song_count: usize,
}
pub struct ArtistUnit;

#[derive(PartialEq, Eq)]
pub enum Window {
    SearchBar,
    Shortcut,
    PaneTab,
    Pane(PaneWindow),
    None,
    Popup,
    Gauge,
}

#[derive(PartialEq, Eq)]
pub enum PaneWindow {
    MusicPane,
    PlaylistPane,
    ArtistPane,
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
    pub music_pane_state: TableState,
    pub playlist_pane_state: TableState,
}

pub struct PaneDivision<const COL_LEN: usize> {
    pub splits: [Constraint; COL_LEN],
    pub spacing: u16,
}

/// Size & Position related config
pub struct GeometryData {
    pub searchbar: Rect,
    pub sidebar: Rect,
    pub gauge: Rect,
    pub panetab: Rect,
    pub musicpane: Rect,
    pub playlistpane: Rect,
    pub artistpane: Rect,
    pub musicpane_division: PaneDivision<3>,
    pub playlistpane_division: PaneDivision<3>,
}
