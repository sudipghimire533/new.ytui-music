use crate::gadgets::state::AppState;
use crate::gadgets::window::Window;
use tui::style::Modifier;
use tui::style::Style;
use tui::widgets::Block;
use tui::widgets::BorderType;
use tui::widgets::Borders;
use tui::widgets::List;
use tui::widgets::ListItem;
use user_config::preferences::{shortcut::Shortcut, theme::Theme};

pub const LIST_ITEMS: [Shortcut; 8] = [
    Shortcut::Trending,
    Shortcut::YoutubeCommunity,
    Shortcut::LikedSongs,
    Shortcut::MyPlaylist,
    Shortcut::Downloaded,
    Shortcut::FollowingArtist,
    Shortcut::Local,
    Shortcut::Search,
];

pub trait ShortcutListAppdata {
    fn is_shortcutlist_active(&self) -> bool;
    fn selected(&self) -> Option<usize>;
    fn get_title(&self) -> &'static str {
        "Shortcuts "
    }
}

impl ShortcutListAppdata for AppState {
    fn is_shortcutlist_active(&self) -> bool {
        self.active_window == Window::Shortcut
    }

    fn selected(&self) -> Option<usize> {
        self.shortcut_list_state.get_ref().selected()
    }
}

pub fn get_shortcut_list<'a, A>(appdata: &A, theme: &Theme) -> List<'a>
where
    A: ShortcutListAppdata,
{
    let list_items = LIST_ITEMS
        .into_iter()
        .enumerate()
        .map(|(i, s)| {
            let item_str = <_ as Into<&'static str>>::into(s);
            let list_color = if appdata.selected() == Some(i) {
                theme.highlight_color
            } else {
                theme.base_color
            };
            ListItem::new(item_str).style(Style::default().fg(list_color.into()))
        })
        .collect::<Vec<_>>();

    let border_style = if appdata.is_shortcutlist_active() {
        Style::default()
            .fg(theme.active_color.into())
            .add_modifier(Modifier::ITALIC)
    } else {
        Style::default()
            .fg(theme.inactive_color.into())
            .add_modifier(Modifier::ITALIC)
    };

    let block = Block::default()
        .title(appdata.get_title())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_type(tui::widgets::BorderType::Rounded)
        .border_style(border_style);

    List::new(list_items).block(block)
}
