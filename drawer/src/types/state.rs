use crate::types::query::FinalQuery;
use crate::types::unit::{ArtistUnit, MusicUnit, PlaylistUnit};
use crate::types::window::Window;
use tui::layout::Constraint;
use tui::layout::Rect;
use tui::widgets::ListState;
use tui::widgets::TableState;

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
