use crate::gadgets::AppState;
use tui::layout::Constraint;
use tui::style::{Color, Style, Modifier};
use tui::widgets::{Block, Borders, Table, TableState, Row};
use user_config::preferences::{shortcut::Shortcut, theme::Theme};
use tui::text::Span;

use super::{Window, QueryResult, MusicUnit};

pub trait MusicpaneAppdata {
    fn is_musicpane_active(&self) -> bool;
    fn selected(&self) -> Option<usize>;
    fn music_list(&self) -> &QueryResult<MusicUnit>;
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
    fn music_list(&self) -> &QueryResult<MusicUnit> {
        &self.music_result
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

    let rows = appdata
        .music_list()
        .list
        .iter()
        .map(|MusicUnit { title, duration, artist }| {
            Row::new(vec![title.clone(), duration.clone(), artist.clone()])
        })
        .collect::<Vec<Row>>();

    let header = Row::new(vec!["Music", "Artist", "Duration"])
        .style(header_style);
    
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