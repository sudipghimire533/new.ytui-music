use tui::widgets::Gauge;
use tui::widgets::Borders;
use tui::widgets::Block;
use tui::widgets::BorderType;
use tui::layout::Alignment;
use tui::style::Style;
use user_config::preferences::theme::Theme;
use crate::gadgets::window::Window;
use crate::gadgets::state::AppState;

pub trait GaugeAppData {
    fn is_gauge_active(&self) -> bool;
    fn get_music_total_duration(&self) -> String;
    fn get_played_music_duration(&self) -> String;
}

impl GaugeAppData for AppState {
    fn is_gauge_active(&self) -> bool {
        self.active_window == Window::Gauge
    }

    fn get_music_total_duration(&self) -> String {
        todo!()
    }

    fn get_played_music_duration(&self) -> String {
        todo!()
    }
}

pub fn get_gauge<'a, A>(appdata: A, theme: &Theme) -> Gauge<'a>
where
    A: GaugeAppData,
{
    let block = Block::default()
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .title(" 00:00 / 00:00 ");

    Gauge::default()
        .gauge_style(Style::default().fg(theme.inactive_color.into()))
        .label("Bartika Eam rai - Bimbakash - mayajastai ( Pokhara Live concert )")
        .percent(21)
        .block(block)
}
