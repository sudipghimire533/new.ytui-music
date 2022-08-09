use crate::gadgets::searchbar::get_searchbar;
use crate::gadgets::searchbar::SearchbarAppdata;
use crate::types::state::GeometryData;
use crate::types::state::ShortcutListState;
use tui::backend::Backend;
use tui::terminal::Frame;
use user_config::preferences::theme::Theme;

use super::gauge::get_gauge;
use super::gauge::GaugeAppData;
use super::musicpane::get_musicpane_list;
use super::musicpane::MusicpaneAppdata;
use super::panetab::get_panetab;
use super::panetab::PanetabAppdata;
use super::playlistpane::get_playlistpane_list;
use super::playlistpane::PlaylistpaneAppdata;
use super::shortcut::get_shortcut_list;
use super::shortcut::ShortcutListAppdata;
use super::state::AppState;
use super::state::MusicPaneState;
use super::state::PanetabState;
use super::state::PlaylistPaneState;
use super::window::PaneWindow;

pub trait Provider<Value> {
    fn provide(&self) -> Value;
}

impl Provider<ShortcutListState> for AppState {
    fn provide(&self) -> ShortcutListState {
        self.shortcut_list_state.clone()
    }
}

impl Provider<PanetabState> for AppState {
    fn provide(&self) -> PanetabState {
        self.panetab_state.clone()
    }
}

impl Provider<MusicPaneState> for AppState {
    fn provide(&self) -> MusicPaneState {
        self.music_pane_state.clone()
    }
}

impl Provider<PlaylistPaneState> for AppState {
    fn provide(&self) -> PlaylistPaneState {
        self.playlist_pane_state.clone()
    }
}

pub fn draw_all_ui<A, B>(frame: &mut Frame<B>, appdata: &A, theme: &Theme, geometrics: GeometryData)
where
    B: Backend,
    A: SearchbarAppdata
        + GaugeAppData
        + ShortcutListAppdata
        + MusicpaneAppdata
        + PlaylistpaneAppdata
        + PanetabAppdata
        + Provider<ShortcutListState>
        + Provider<PanetabState>
        + Provider<MusicPaneState>
        + Provider<PlaylistPaneState>,
{
    let searchbar_rect = geometrics.searchbar;
    if searchbar_rect.area() > 1 {
        let searchbar = get_searchbar(appdata, theme);
        frame.render_widget(searchbar, searchbar_rect);
    }

    let gauge_rect = geometrics.gauge;
    if searchbar_rect.area() > 1 {
        let gauge = get_gauge(appdata, theme);
        frame.render_widget(gauge, gauge_rect)
    }

    let sidebar_rect = geometrics.sidebar;
    if sidebar_rect.area() > 1 {
        let sidebar = get_shortcut_list(appdata, theme);
        let mut sidebar_state = <A as Provider<ShortcutListState>>::provide(appdata);
        frame.render_stateful_widget(sidebar, sidebar_rect, sidebar_state.get_mut_ref())
    }

    let panetab_rect = geometrics.panetab;
    if panetab_rect.area() > 1 {
        let panetab = get_panetab(appdata, theme);
        frame.render_widget(panetab, panetab_rect)
    }

    let panetab_state = <A as Provider<PanetabState>>::provide(appdata);
    let active_pane =
        PaneWindow::try_from_index(panetab_state.selected).unwrap_or(PaneWindow::MusicPane);

    match active_pane {
        PaneWindow::MusicPane => {
            let musicpane_rect = geometrics.musicpane;
            if musicpane_rect.area() > 1 {
                let mut musicpane_state = <A as Provider<MusicPaneState>>::provide(appdata);
                let musicpane = get_musicpane_list(appdata, &geometrics, theme);
                frame.render_stateful_widget(
                    musicpane,
                    musicpane_rect,
                    musicpane_state.get_mut_ref(),
                );
            }
        }

        PaneWindow::PlaylistPane => {
            let playlistpane_rect = geometrics.playlistpane;
            if playlistpane_rect.area() > 1 {
                let mut playlistpane_state = <A as Provider<PlaylistPaneState>>::provide(appdata);
                let playlistpane = get_playlistpane_list(appdata, &geometrics, theme);
                frame.render_stateful_widget(
                    playlistpane,
                    playlistpane_rect,
                    playlistpane_state.get_mut_ref(),
                );
            }
        }

        PaneWindow::ArtistPane => {
            unimplemented!()
        }
    }
}
