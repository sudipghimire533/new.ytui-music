use crate::gadgets::state::AppState;
use crate::gadgets::window::Window;
use tui::style::Style;
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::Block;
use tui::widgets::BorderType;
use tui::widgets::Borders;
use tui::widgets::Tabs;
use user_config::preferences::theme::Theme;

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
    let selected_tab = appdata.selected();
    let items = ["Music", "Playlist", "Artist"]
        .into_iter()
        .enumerate()
        .map(|(i, label)| {
            let label_color = if i == selected_tab {
                theme.base_color
            } else {
                theme.active_color
            };
            Spans::from(Span::styled(label, Style::default().fg(label_color.into())))
        })
        .collect::<Vec<_>>();

    let border_style = if appdata.is_panetab_active() {
        Style::default().fg(theme.active_color.into())
    } else {
        Style::default().fg(theme.inactive_color.into())
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_style)
        .title(appdata.get_title());

    Tabs::new(items).block(block)
}
