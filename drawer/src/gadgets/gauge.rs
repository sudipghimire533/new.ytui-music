use crate::gadgets::state::AppState;
use crate::gadgets::state::PlayerInfo;
use crate::gadgets::window::Window;
use tui::layout::Alignment;
use tui::style::Style;
use tui::widgets::Block;
use tui::widgets::BorderType;
use tui::widgets::Borders;
use tui::widgets::Gauge;
use user_config::preferences::theme::Theme;

pub trait GaugeAppData {
    fn is_gauge_active(&self) -> bool;
    fn music_title(&self) -> String;
    fn music_total_duration(&self) -> String;
    fn played_music_duration(&self) -> String;
}

impl GaugeAppData for AppState {
    fn is_gauge_active(&self) -> bool {
        self.active_window == Window::Gauge
    }
    fn music_total_duration(&self) -> String {
        self.playing_track_duration()
    }
    fn played_music_duration(&self) -> String {
        self.playing_track_completed()
    }
    fn music_title(&self) -> String {
        self.playing_track_title()
    }
}

// TODO:
// receive actual Duration type
fn get_played_percent(total: String, played: String) -> u16 {
    let (_, _) = (total, played);
    21
}

pub fn get_gauge<'a, A>(appdata: &A, theme: &Theme) -> Gauge<'a>
where
    A: GaugeAppData,
{
    let playing_title = appdata.music_title();
    let combined_duration = format!(
        "{played} / {total}",
        played = appdata.played_music_duration(),
        total = appdata.music_total_duration()
    );
    let base_style = Style::default().fg(theme.base_color.into());
    let gauge_style = Style::default().fg(theme.inactive_color.into());
    let played_percent = get_played_percent(String::new(), String::new());

    let block = Block::default()
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .title(combined_duration);

    Gauge::default()
        .gauge_style(gauge_style)
        .style(base_style)
        .label(playing_title)
        .percent(played_percent)
        .block(block)
}
