#[derive(PartialEq, Eq)]
pub enum Window {
    SearchBar,
    Shortcut,
    PaneTab(PaneWindow),
    Pane(PaneWindow),
    Popup,
    Gauge,
    None,
}

impl Window {
    pub fn next(&self) -> Self {
        match self {
            Window::SearchBar => Window::Shortcut,
            Window::Shortcut => Window::PaneTab(PaneWindow::first()),
            Window::PaneTab(pane_window) => match pane_window.next() {
                Some(new_pane) => Window::PaneTab(new_pane),
                None => Window::Pane(PaneWindow::MusicPane),
            },
            Window::Pane(pane_window) => match pane_window.next() {
                Some(new_pane) => Window::Pane(new_pane),
                None => Window::Gauge,
            },
            Window::Popup => Window::SearchBar,
            Window::Gauge => Window::SearchBar,
            Window::None => Window::SearchBar,
        }
    }
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

    pub fn first() -> Self {
        PaneWindow::MusicPane
    }

    pub fn last() -> Self {
        PaneWindow::ArtistPane
    }

    pub fn next(self) -> Option<Self> {
        Self::try_from_index(self.into_index() + 1)
    }
}
