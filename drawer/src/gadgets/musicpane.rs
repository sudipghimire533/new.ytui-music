use crate::gadgets::AppState;
use tui::style::{Color, Style, Modifier};
use tui::widgets::{Block, Borders, Table, TableState};
use user_config::preferences::{shortcut::Shortcut, theme::Theme};
use tui::text::Span;

use super::Window;

pub trait MusicpaneAppdata {
    fn is_musicpane_active(&self) -> bool;
    fn get_title(&self) -> &'static str {
        "Musics "
    }
}

impl MusicpaneAppdata for AppState {
    fn is_musicpane_active(&self) -> bool {
        self.active_window == Window::MusicPane
    }
}

pub fn get_musicpane_list<'a, A>(appdata: A, theme: &Theme) -> Table<'a>
where
    A: MusicpaneAppdata,
{
    let block_title: Span;
    let border_style: Style;
    if appdata.is_musicpane_active() {
        block_title = Span::styled(appdata.get_title(), Default::default());
        border_style = Style::default().fg(theme.active_color.into());
    } else {
        block_title = Span::styled(appdata.get_title(), Default::default());
        border_style = Style::default().fg(theme.inactive_color.into());
    }

    let block = Block::default()
        .border_type(tui::widgets::BorderType::Rounded)
        .title(block_title)
        .borders(Borders::ALL)
        .border_style(border_style);

    let rows = vec![];

    Table::new(rows).block(block)
}
