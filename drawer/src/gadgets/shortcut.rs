use tui::style::Style;
use tui::widgets::Borders;
use tui::widgets::BorderType;
use tui::widgets::Block;
use tui::widgets::List;
use tui::widgets::ListItem;
use user_config::preferences::{shortcut::Shortcut, theme::Theme};
use crate::gadgets::window::Window;
use crate::gadgets::state::AppState;

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
        self.shortcut_list_state.selected()
    }
}

pub fn get_shortcut_list<'a, A>(appdata: A, theme: &Theme) -> List<'a>
where
    A: ShortcutListAppdata,
{
    const LIST_ITEMS: [Shortcut; 8] = [
        Shortcut::Trending,
        Shortcut::YoutubeCommunity,
        Shortcut::LikedSongs,
        Shortcut::MyPlaylist,
        Shortcut::Downloaded,
        Shortcut::FollowingArtist,
        Shortcut::Local,
        Shortcut::Search,
    ];

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

    let border_style: Style;
    if appdata.is_shortcutlist_active() {
        border_style = Style::default().fg(theme.active_color.into());
    } else {
        border_style = Style::default().fg(theme.inactive_color.into());
    }

    let block = Block::default()
        .title(appdata.get_title())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_type(tui::widgets::BorderType::Rounded)
        .border_style(border_style);

    List::new(list_items).block(block)
}
