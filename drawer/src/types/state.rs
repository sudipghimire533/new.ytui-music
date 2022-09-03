use crate::types::query::FinalQuery;
use crate::types::unit::{ArtistUnit, MusicUnit, PlaylistUnit};
use crate::types::window::Window;
use tui::layout::Constraint;
use tui::layout::Rect;
use tui::widgets::ListState;
use tui::widgets::TableState;

use super::query::Query;

macro_rules! make_wrapper {
    ($name: ident, $inner: ident) => {
        // Todo:
        /// Make #[derive] statement as input of this arm
        #[derive(Clone)]
        pub struct $name(pub $inner);
        impl $name {
            pub fn get(self) -> $inner {
                self.0
            }
            pub fn get_ref(&self) -> &$inner {
                &self.0
            }
            pub fn get_mut_ref(&mut self) -> &mut $inner {
                &mut self.0
            }
        }
    };
}

pub trait PlayerInfo {
    fn playing_track_title(&self) -> String;
    // TODO: return more Duration specific type rather than String
    fn playing_track_duration(&self) -> String;
    fn playing_track_completed(&self) -> String;
}

impl PlayerInfo for AppState {
    fn playing_track_title(&self) -> String {
        "Some title".to_string()
    }
    fn playing_track_duration(&self) -> String {
        "Some duration".to_string()
    }
    fn playing_track_completed(&self) -> String {
        "Some duration".to_string()
    }
}

pub struct QueryResult<T> {
    pub query: FinalQuery,
    pub list: Vec<T>,
}

make_wrapper!(ShortcutListState, ListState);
make_wrapper!(MusicPaneState, TableState);
make_wrapper!(PlaylistPaneState, TableState);

#[derive(Clone)]
pub struct PanetabState {
    pub selected: usize,
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
    pub panetab_state: PanetabState,
    pub shortcut_list_state: ShortcutListState,
    pub music_pane_state: MusicPaneState,
    pub playlist_pane_state: PlaylistPaneState,
}

/// Default Appstate when application is freshly started
impl Default for AppState {
    fn default() -> Self {
        AppState {
            altering_query: String::new(),
            music_result: QueryResult {
                query: Query::Nothing.get_final_query(),
                list: [].into(),
            },
            artist_result: QueryResult {
                query: Query::Nothing.get_final_query(),
                list: [].into(),
            },
            playlist_result: QueryResult {
                query: Query::Nothing.get_final_query(),
                list: [].into(),
            },
            active_window: Window::SearchBar,
            panetab_state: PanetabState { selected: 0 },
            shortcut_list_state: ShortcutListState(Default::default()),
            music_pane_state: MusicPaneState(Default::default()),
            playlist_pane_state: PlaylistPaneState(Default::default()),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PaneDivision<const COL_LEN: usize> {
    pub splits: [Constraint; COL_LEN],
    pub spacing: u16,
}

/// Size & Position related config
#[derive(Debug, Clone, Eq, PartialEq)]
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
