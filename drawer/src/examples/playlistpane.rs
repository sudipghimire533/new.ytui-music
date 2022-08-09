mod common;
use common::*;
use drawer::gadgets::{
    playlistpane::{self, PlaylistpaneGeometry},
    PlaylistUnit,
};
use tui::{layout::Rect, widgets::TableState};

struct ExampleGeometry;
struct ExamplePlaylistpaneAppdata {
    playlist_list: Vec<PlaylistUnit>,
}

impl ExamplePlaylistpaneAppdata {
    fn new_filled() -> Self {
        let rows = [
            (
                "Best of South East Asia Top 100",
                "Spotify",
                100
            ),
            ("Album Bimbakash", "Bartika Eam Rai", 17),
            (
                "Best soft songs to chill at while doing homework in late night alone",
                "Rachana Dahal",
                132,
            ),
            ("Gems - Rachana Dahal - Aagya", "Rachana Dahal", usize::MAX),
            (
                "Gaming sounds - Mux of GOT, COC, PubG, Freefire, War and Thunder and so on",
                "Gaing vibes",
                28,
            ),
        ]
        .into_iter()
        .collect::<Vec<_>>();
        let playlist_list = rows
            .clone()
            .into_iter()
            .chain(rows.clone().into_iter())
            .chain(rows.clone().into_iter())
            .chain(rows.clone().into_iter())
            .chain(rows.clone().into_iter())
            .map(|(title, creator, count)| PlaylistUnit {
                song_count: count,
                title: title.to_string(),
                creator: creator.to_string(),
            })
            .collect::<Vec<PlaylistUnit>>();

        Self { playlist_list }
    }
}

impl PlaylistpaneGeometry for ExampleGeometry {
    fn column_division(&self) -> &[Constraint] {
        &[
            Constraint::Length(4),
            Constraint::Percentage(70),
            Constraint::Percentage(15),
        ]
    }
    fn column_spacing(&self) -> u16 {
        2
    }
}

impl playlistpane::PlaylistpaneAppdata for ExamplePlaylistpaneAppdata {
    fn is_playlistpane_active(&self) -> bool {
        true
    }

    fn selected(&self) -> Option<usize> {
        Some(4)
    }

    fn playlist_list(&self) -> &[PlaylistUnit] {
        &self.playlist_list
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    entrypoint(draw_playlistpane)
}

fn draw_playlistpane<B: Backend>(f: &mut Frame<B>) {
    let theme = get_default_theme();
    let shortcut = playlistpane::get_playlistpane_list(
        ExamplePlaylistpaneAppdata::new_filled(),
        &ExampleGeometry,
        &theme,
    );
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
