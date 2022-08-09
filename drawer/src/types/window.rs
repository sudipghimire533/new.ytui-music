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

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum PaneWindow {
    MusicPane,
    PlaylistPane,
    ArtistPane,
}

impl PaneWindow {
    pub fn into_index(self) -> usize {
        match self {
            PaneWindow::MusicPane => 0,
            PaneWindow::PlaylistPane => 1,
            PaneWindow::ArtistPane => 2,
        }
    }

    pub fn try_from_index(index: usize) -> Option<Self> {
        if index == 0 {
            Some(PaneWindow::MusicPane)
        } else if index == 1 {
            Some(PaneWindow::PlaylistPane)
        } else if index == 2 {
            Some(PaneWindow::ArtistPane)
        } else {
            None
        }
    }
}
