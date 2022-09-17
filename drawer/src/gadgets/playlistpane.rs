use crate::gadgets::state::AppState;
use crate::gadgets::state::GeometryData;
use crate::gadgets::unit::PlaylistUnit;
use crate::gadgets::window::PaneWindow;
use tui::layout::Constraint;
use tui::style::Modifier;
use tui::style::Style;

use tui::widgets::Block;
use tui::widgets::BorderType;
use tui::widgets::Borders;
use tui::widgets::Row;
use tui::widgets::Table;
use user_config::preferences::theme::Theme;

pub trait PlaylistpaneAppdata {
    fn is_playlistpane_active(&self) -> bool;
    fn selected(&self) -> Option<usize>;
    fn playlist_list(&self) -> &[PlaylistUnit];
    fn get_title(&self) -> &'static str {
        "Playlists "
    }
}

pub trait PlaylistpaneGeometry {
    fn column_division(&self) -> &[Constraint];
    fn column_spacing(&self) -> u16;
}

impl PlaylistpaneAppdata for AppState {
    fn is_playlistpane_active(&self) -> bool {
        self.panetab_state.active_tab == PaneWindow::PlaylistPane
    }
    fn selected(&self) -> Option<usize> {
        self.playlist_pane_state.get_ref().selected()
    }
    fn playlist_list(&self) -> &[PlaylistUnit] {
        &self.playlist_result.list
    }
}

impl PlaylistpaneGeometry for GeometryData {
    fn column_division(&self) -> &[Constraint] {
        &self.playlistpane_division.splits
    }
    fn column_spacing(&self) -> u16 {
        self.playlistpane_division.spacing
    }
}

pub fn get_playlistpane_list<'a, A, G>(appdata: &A, geometry: &'a G, theme: &Theme) -> Table<'a>
where
    A: PlaylistpaneAppdata,
    G: PlaylistpaneGeometry,
{
    let border_style = if appdata.is_playlistpane_active() {
        Style::default()
            .fg(theme.active_color.into())
            .add_modifier(Modifier::ITALIC)
    } else {
        Style::default()
            .fg(theme.inactive_color.into())
            .add_modifier(Modifier::ITALIC)
    };

    let header_style = Style::default()
        .fg(theme.inactive_color.into())
        .add_modifier(Modifier::BOLD);
    let base_style = Style::default().fg(theme.base_color.into());
    let highlight_style = Style::default().fg(theme.highlight_color.into());

    let block = Block::default()
        .border_type(tui::widgets::BorderType::Rounded)
        .title(appdata.get_title())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_style);

    let rows = appdata
        .playlist_list()
        .iter()
        .map(
            |PlaylistUnit {
                 song_count,
                 title,
                 creator: artist,
             }| {
                Row::new(vec![song_count.to_string(), title.clone(), artist.clone()])
            },
        )
        .collect::<Vec<Row>>();

    let header = Row::new(vec!["count", "Title", "Created by"]).style(header_style);
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
