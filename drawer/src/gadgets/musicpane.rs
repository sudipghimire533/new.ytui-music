use crate::gadgets::AppState;
use crate::gadgets::GeometryData;
use tui::layout::Constraint;
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders, Row, Table, TableState};
use user_config::preferences::{shortcut::Shortcut, theme::Theme};

use super::PaneWindow;
use super::{MusicUnit, QueryResult, Window};

pub trait MusicpaneAppdata {
    fn is_musicpane_active(&self) -> bool;
    fn selected(&self) -> Option<usize>;
    fn music_list(&self) -> &[MusicUnit];
    fn get_title(&self) -> &'static str {
        "Musics "
    }
}

pub trait MusicpaneGeometry {
    fn column_division(&self) -> &[Constraint];
    fn column_spacing(&self) -> u16;
}

impl MusicpaneAppdata for AppState {
    fn is_musicpane_active(&self) -> bool {
        self.active_window == Window::Pane(PaneWindow::MusicPane)
    }
    fn selected(&self) -> Option<usize> {
        self.music_pane_state.selected()
    }
    fn music_list(&self) -> &[MusicUnit] {
        &self.music_result.list
    }
}

impl MusicpaneGeometry for GeometryData {
    fn column_division(&self) -> &[Constraint] {
        &self.musicpane_division.splits
    }
    fn column_spacing(&self) -> u16 {
        self.musicpane_division.spacing
    }
}

pub fn get_musicpane_list<'a, A, G>(appdata: A, geometry: &'a G, theme: &Theme) -> Table<'a>
where
    A: MusicpaneAppdata,
    G: MusicpaneGeometry,
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

    let header_style = Style::default()
        .fg(theme.inactive_color.into())
        .add_modifier(Modifier::BOLD);
    let base_style = Style::default().fg(theme.base_color.into());
    let highlight_style = Style::default().fg(theme.highlight_color.into());

    let block = Block::default()
        .border_type(tui::widgets::BorderType::Rounded)
        .title(block_title)
        .borders(Borders::ALL)
        .border_style(border_style);

    let rows = appdata
        .music_list()
        .iter()
        .map(
            |MusicUnit {
                 title,
                 duration,
                 artist,
             }| { Row::new(vec![title.clone(), artist.clone(), duration.clone()]) },
        )
        .collect::<Vec<Row>>();

    let header = Row::new(vec!["Music", "Artist", "Duration"]).style(header_style);
    let widths = geometry.column_division();
    let col_spacing = geometry.column_spacing();

    Table::new(rows)
        .column_spacing(col_spacing)
        .widths(widths)
        .header(header)
        .style(base_style)
        .highlight_style(highlight_style)
        .block(block)
}
