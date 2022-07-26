use crate::gadgets::state::AppState;
use crate::gadgets::window::Window;
use tui::style::Modifier;
use tui::style::Style;
use tui::text::{Span, Spans, Text};
use tui::widgets::Block;
use tui::widgets::BorderType;
use tui::widgets::Borders;
use tui::widgets::Paragraph;
use user_config::preferences::theme::Theme;

pub trait SearchbarAppdata {
    fn is_searchbar_active(&self) -> bool;
    fn get_altering_query(&self) -> &str;
    fn get_title(&self) -> &'static str {
        "Search "
    }
    fn get_cursor(&self) -> &'static str {
        "/"
    }
}

impl SearchbarAppdata for AppState {
    fn is_searchbar_active(&self) -> bool {
        self.active_window == Window::SearchBar
    }
    fn get_altering_query(&self) -> &str {
        &self.altering_query
    }
}

pub fn get_searchbar<'c, A>(appdata: &A, theme: &Theme) -> Paragraph<'c>
where
    A: SearchbarAppdata,
{
    let cursor_style;
    let border_style;
    if appdata.is_searchbar_active() {
        border_style = Style::default()
            .fg(theme.active_color.into())
            .add_modifier(Modifier::ITALIC);
        cursor_style = Style::default()
            .add_modifier(Modifier::RAPID_BLINK)
            .fg(theme.highlight_color.into());
    } else {
        border_style = Style::default()
            .fg(theme.inactive_color.into())
            .add_modifier(Modifier::ITALIC);
        cursor_style = Style::default().fg(theme.inactive_color.into());
    }

    let text: Text = Spans::from(vec![
        Span::styled(
            appdata.get_altering_query().to_string(),
            Style::default().fg(theme.base_color.into()),
        ),
        Span::styled(appdata.get_cursor(), cursor_style),
    ])
    .into();

    let block = Block::default()
        .border_type(tui::widgets::BorderType::Rounded)
        .title(appdata.get_title())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_style);

    Paragraph::new(text).block(block)
}
