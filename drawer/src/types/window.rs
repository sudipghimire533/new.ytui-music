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
