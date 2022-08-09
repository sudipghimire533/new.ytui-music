mod common;
use common::*;
use drawer::gadgets::{musicpane, FinalQuery, MusicUnit, Query, QueryResult};
use tui::{layout::Rect, widgets::TableState};

struct ExampleMusicpaneAppdata {
    result: QueryResult<MusicUnit>,
}

impl ExampleMusicpaneAppdata {
    fn new_filled() -> Self {
        let rows = [
            [
                "Gems of Nepal, Session 323 - Rachana Dahal - Aagya",
                "Rachana Dahal",
                "03:43",
            ],
            ["Bimbakash - Bimbakash", "I am sudip ghimire x 533", "03:43"],
            [
                "Gems of Nepal, Session 323 - Rachana Dahal - Aagya",
                "Rachana Dahal",
                "03:43",
            ],
            ["Gems - Rachana Dahal - Aagya", "Rachana Dahal", "01:03:43"],
            [
                "Come and get your love - Gurdain of Galaxy",
                "From movie Gurdain of galaxy",
                "05:03",
            ],
        ]
        .into_iter()
        .collect::<Vec<_>>();
        let list = rows
            .clone()
            .into_iter()
            .chain(rows.clone().into_iter())
            .chain(rows.clone().into_iter())
            .chain(rows.clone().into_iter())
            .chain(rows.clone().into_iter())
            .map(|[title, artist, duration]| MusicUnit {
                title: title.to_string(),
                artist: artist.to_string(),
                duration: duration.to_string(),
            })
            .collect::<Vec<MusicUnit>>();
        
        Self {
            result: QueryResult {
                query: Query::LikedMusic.get_final_query(),
                list
            }
        }
    }
}

impl musicpane::MusicpaneAppdata for ExampleMusicpaneAppdata {
    fn is_musicpane_active(&self) -> bool {
        true
    }

    fn selected(&self) -> Option<usize> {
        Some(4)
    }

    fn music_list(&self) -> &drawer::gadgets::QueryResult<drawer::gadgets::MusicUnit> {
        &self.result
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    entrypoint(draw_musicpane)
}

fn draw_musicpane<B: Backend>(f: &mut Frame<B>) {
    let theme = get_default_theme();
    let shortcut = musicpane::get_musicpane_list(ExampleMusicpaneAppdata::new_filled(), &theme);
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
