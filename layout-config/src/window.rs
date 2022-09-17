use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Copy)]
pub enum Window {
    SearchBar,
    Shortcut,
    PaneTab,
    PaneWindow,
    Popup,
    Gauge,
    None,
}

impl std::fmt::Display for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let window =
            serde_json::to_string(self).expect("to_string call to simple enum should not fail");
        write!(f, "{window}")
    }
}

impl Window {
    pub fn next(&self) -> Option<Self> {
        let next = match self {
            Window::SearchBar => Window::Shortcut,
            Window::Shortcut => Window::PaneTab,
            Window::PaneTab => Window::PaneWindow,
            Window::PaneWindow => Window::Gauge,
            Window::Popup => Window::SearchBar,
            Window::Gauge => return None,
            Window::None => return None,
        };
        Some(next)
    }

    pub fn prev(&self) -> Option<Self> {
        let prev = match self {
            Window::SearchBar => return None,
            Window::Shortcut => Window::SearchBar,
            Window::PaneTab => Window::Shortcut,
            Window::PaneWindow => Window::PaneTab,
            Window::Popup => Window::Gauge,
            Window::Gauge => Window::PaneWindow,
            Window::None => return None,
        };
        Some(prev)
    }

    pub fn first() -> Self {
        Window::SearchBar
    }

    pub fn last() -> Self {
        Window::Gauge
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum PaneWindow {
    MusicPane = 0,
    PlaylistPane = 1,
    ArtistPane = 2,
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
        Self::try_from_index(self.into_index().checked_add(1)?)
    }

    pub fn prev(self) -> Option<Self> {
        Self::try_from_index(self.into_index().checked_sub(1)?)
    }
}
