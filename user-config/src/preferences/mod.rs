pub mod pane;
pub mod pane_tab;
pub mod searchbar;
pub mod shortcut;
pub mod theme;
use pane::PanePreferences;
use pane_tab::PaneTabPreferences;
use searchbar::SearchbarPreferences;
use shortcut::ShortcutPreferences;
use theme::Theme;

pub struct WidgetPreferences {
    /// if option is NONE, music pane will not be shown in ui
    pub music_pane: Option<PanePreferences>,
    pub artist_pane: Option<PanePreferences>,
    pub playlist_pane: Option<PanePreferences>,
    pub pane_tab: Option<PaneTabPreferences>,
    pub shortcuts: Option<ShortcutPreferences>,
    pub searchbar: Option<SearchbarPreferences>,
    pub theme: Theme,
}
