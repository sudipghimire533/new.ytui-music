use crate::gadgets::state::AppState;
use crate::gadgets::window::Window;
use tui::style::Modifier;
use tui::style::Style;
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::Block;
use tui::widgets::BorderType;
use tui::widgets::Borders;
use tui::widgets::Tabs;
use user_config::preferences::theme::Theme;

pub const TAB_NAMES: [&str; 3] = ["Music", "Playlist", "Artist"];
pub const SEPERATOR: &str = "|";

pub fn get_preferred_width() -> usize {
    TAB_NAMES.into_iter().fold(0, |acc, t| acc + Span::from(t).width()) // width of text
        + Span::from(SEPERATOR).width() * ( TAB_NAMES.len() - 1 ) //  width of two SEPERATOR
        + Span::from(" ").width() * TAB_NAMES.len() * 2 // space will be on both side of seperator
        + 2 // two for border
}

pub trait PanetabAppdata {
    fn is_panetab_active(&self) -> bool;
    fn get_title(&self) -> &'static str {
        "Filter "
    }
    fn selected(&self) -> usize;
}

impl PanetabAppdata for AppState {
    fn is_panetab_active(&self) -> bool {
        self.active_window == Window::PaneTab
    }
    fn selected(&self) -> usize {
        self.panetab_state.selected
    }
}

pub fn get_panetab<'a, A>(appdata: &A, theme: &Theme) -> Tabs<'a>
where
    A: PanetabAppdata,
{
    //let selected_tab = appdata.selected();
    let items = TAB_NAMES
        .into_iter()
        .map(|tab_name| Spans::from(Span::from(tab_name)))
        .collect::<Vec<_>>();

    let selected = appdata.selected();
    let base_style = Style::default().fg(theme.base_color.into());
    let highlight_style = Style::default().fg(theme.active_color.into());
    let border_style = if appdata.is_panetab_active() {
        Style::default()
            .fg(theme.active_color.into())
            .add_modifier(Modifier::ITALIC)
    } else {
        Style::default()
            .fg(theme.inactive_color.into())
            .add_modifier(Modifier::ITALIC)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_style)
        .title(appdata.get_title());

    Tabs::new(items)
        .divider(SEPERATOR)
        .highlight_style(highlight_style)
        .style(base_style)
        .block(block)
        .select(selected)
}
