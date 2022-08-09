use crate::gadgets::AppState;
use tui::layout::Constraint;
use tui::style::{Color, Style, Modifier};
use tui::widgets::{Block, Borders, Table, TableState, Row};
use user_config::preferences::{shortcut::Shortcut, theme::Theme};
use tui::text::Span;

use super::Window;

pub trait MusicpaneAppdata {
    fn is_musicpane_active(&self) -> bool;
    fn selected(&self) -> Option<usize>;
    fn get_title(&self) -> &'static str {
        "Musics "
    }
}

impl MusicpaneAppdata for AppState {
    fn is_musicpane_active(&self) -> bool {
        self.active_window == Window::MusicPane
    }
    fn selected(&self) -> Option<usize> {
        self.music_pane_state.selected()
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

    let header_style = Style::default().fg(theme.inactive_color.into()).add_modifier(Modifier::BOLD);
    let base_style = Style::default().fg(theme.base_color.into());
    let highlight_style = Style::default().fg(theme.highlight_color.into());

    let block = Block::default()
        .border_type(tui::widgets::BorderType::Rounded)
        .title(block_title)
        .borders(Borders::ALL)
        .border_style(border_style);

    let header = Row::new(vec!["Music", "Artist", "Duration"])
        .style(header_style);
    let rows = [
        Row::new(vec!["Gems of Nepal, Session 323 - Rachana Dahal - Aagya", "Rachana Dahal", "03:43"]),
        Row::new(vec!["Bimbakash - Bimbakash", "I am sudip ghimire x 533", "03:43"]),
        Row::new(vec!["Gems of Nepal, Session 323 - Rachana Dahal - Aagya", "Rachana Dahal", "03:43"]),
        Row::new(vec!["Gems - Rachana Dahal - Aagya", "Rachana Dahal", "01:03:43"]),
        Row::new(vec!["Come and get your love - Gurdain of Galaxy", "From movie Gurdain of galaxy", "05:03"])
    ]
    .into_iter()
    .collect::<Vec<_>>();
    let rows = rows
        .clone()
        .into_iter()
        .chain(rows.clone().into_iter())
        .chain(rows.clone().into_iter())
        .chain(rows.clone().into_iter())
        .chain(rows.clone().into_iter())
        .collect::<Vec<Row>>();

    Table::new(rows)
        .column_spacing(1)
        .widths(&[
            Constraint::Percentage(60),
            Constraint::Percentage(25),
            Constraint::Percentage(15),
        ])
        .header(header)
        .style(base_style)
        .highlight_style(highlight_style)
        .block(block)
}
