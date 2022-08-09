use tui::style::Style;
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::Block;
use tui::widgets::BorderType;
use tui::widgets::Borders;
use tui::widgets::Tabs;
use user_config::preferences::theme::Theme;
use crate::gadgets::state::AppState;
use crate::gadgets::window::Window;

pub trait PanetabAppdata {
    fn is_panetab_active(&self) -> bool;
    fn get_title(&self) -> &'static str {
        "Filter "
    }
}

impl PanetabAppdata for AppState {
    fn is_panetab_active(&self) -> bool {
        self.active_window == Window::PaneTab
    }
}

pub fn get_panetab<'a, A>(appdata: A, theme: &Theme) -> Tabs<'a>
where
    A: PanetabAppdata,
{
    let items = ["Music", "Playlist", "Artist"]
        .into_iter()
        .map(|label| {
            let label_color = theme.base_color.into();
            Spans::from(Span::styled(label, Style::default().fg(label_color)))
        })
        .collect::<Vec<_>>();

    let border_style: Style;
    if appdata.is_panetab_active() {
        border_style = Style::default().fg(theme.active_color.into());
    } else {
        border_style = Style::default().fg(theme.inactive_color.into());
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_style)
        .title(appdata.get_title());

    Tabs::new(items).block(block)
}
