mod common;
use common::*;
use drawer::gadgets::musicpane;
use tui::{layout::Rect, widgets::TableState};

struct ExampleMusicpaneAppdata;

impl musicpane::MusicpaneAppdata for ExampleMusicpaneAppdata {
    fn is_musicpane_active(&self) -> bool {
        true
    }

    fn selected(&self) -> Option<usize> {
        Some(4)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    entrypoint(draw_musicpane)
}

fn draw_musicpane<B: Backend>(f: &mut Frame<B>) {
    let theme = get_default_theme();
    let shortcut = musicpane::get_musicpane_list(ExampleMusicpaneAppdata, &theme);
    let place = Rect {
        x: 15,
        y: 7,
        height: f.size().height - (7 + 4),
        width: f.size().width - (15 + 10),
    };
    let mut table_state = TableState::default();
    table_state.select(Some(4));

    f.render_stateful_widget(shortcut, place, &mut table_state);
}
