mod common;
use common::*;
use drawer::gadgets::gauge::GaugeAppData;
use drawer::gadgets::musicpane::MusicpaneAppdata;
use drawer::gadgets::panetab::get_preferred_width as panetab_preferred_width;
use drawer::gadgets::panetab::PanetabAppdata;
use drawer::gadgets::playlistpane::PlaylistpaneAppdata;
use drawer::gadgets::searchbar::SearchbarAppdata;
use drawer::gadgets::shortcut::ShortcutListAppdata;
use drawer::gadgets::state::GeometryData;
use drawer::gadgets::state::MusicPaneState;
use drawer::gadgets::state::PaneDivision;
use drawer::gadgets::state::PanetabState;
use drawer::gadgets::state::PlaylistPaneState;
use drawer::gadgets::state::ShortcutListState;
use drawer::gadgets::ui::draw_all_ui;
use drawer::gadgets::ui::Provider;
use drawer::gadgets::unit::MusicUnit;
use drawer::gadgets::unit::PlaylistUnit;
use drawer::types::window::PaneWindow;
use tui::layout::Constraint;
use tui::layout::Rect;
use tui::widgets::ListState;
use tui::widgets::TableState;
use user_config::preferences::theme::Theme;
use user_config::styles::color::RGB;

struct ExampleAppdata {
    playlist_list: Vec<PlaylistUnit>,
}

impl ExampleAppdata {
    fn get_filled() -> Self {
        let rows = [
            ("Best of South East Asia Top 100", "Spotify", 100),
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

        ExampleAppdata { playlist_list }
    }
}

impl SearchbarAppdata for ExampleAppdata {
    fn get_altering_query(&self) -> &str {
        "Searching something.."
    }
    fn is_searchbar_active(&self) -> bool {
        false
    }
}

impl GaugeAppData for ExampleAppdata {
    fn is_gauge_active(&self) -> bool {
        false
    }
    fn played_music_duration(&self) -> String {
        "04:56".to_string()
    }
    fn music_total_duration(&self) -> String {
        "08:03".to_string()
    }
    fn music_title(&self) -> String {
        "Kabira -[Slowed+Reverb]- Ya Jawaani Hai Deewani ||Last Hope".to_string()
    }
}

impl PanetabAppdata for ExampleAppdata {
    fn selected(&self) -> usize {
        1
    }
    fn is_panetab_active(&self) -> bool {
        false
    }
}

impl ShortcutListAppdata for ExampleAppdata {
    fn selected(&self) -> Option<usize> {
        Some(3)
    }
    fn is_shortcutlist_active(&self) -> bool {
        false
    }
}

impl MusicpaneAppdata for ExampleAppdata {
    fn is_musicpane_active(&self) -> bool {
        false
    }
    fn music_list(&self) -> &[MusicUnit] {
        &[]
    }
    fn selected(&self) -> Option<usize> {
        None
    }
}

impl PlaylistpaneAppdata for ExampleAppdata {
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

impl Provider<ShortcutListState> for ExampleAppdata {
    fn provide(&self) -> ShortcutListState {
        let mut list_state = ListState::default();
        list_state.select(Some(3));
        ShortcutListState(list_state)
    }
}

impl Provider<PanetabState> for ExampleAppdata {
    fn provide(&self) -> PanetabState {
        PanetabState {
            active_tab: PaneWindow::MusicPane,
        }
    }
}

impl Provider<MusicPaneState> for ExampleAppdata {
    fn provide(&self) -> MusicPaneState {
        MusicPaneState(TableState::default())
    }
}

impl Provider<PlaylistPaneState> for ExampleAppdata {
    fn provide(&self) -> PlaylistPaneState {
        let mut table_state = TableState::default();
        table_state.select(Some(4));
        PlaylistPaneState(table_state)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    entrypoint(draw_playlistpane)
}

fn draw_playlistpane<B: tui::backend::Backend>(f: &mut tui::Frame<B>) {
    let screen_size = f.size();
    if screen_size.height < 20 {
        panic!(
            "Your terminal height {} is too small to draw ui. Maxamize your terminal or zoom out",
            screen_size.height
        );
    }
    if screen_size.width < 70 {
        panic!(
            "Your terminal width {} is too small to draw ui. Maxamize your terminal or zoom out",
            screen_size.width
        );
    }

    let searchbar_rect = Rect {
        x: 0,
        y: 0,
        height: 3,
        width: screen_size.width,
    };
    let gauge_rect = Rect {
        x: 0,
        y: screen_size.height - 3,
        height: 3,
        width: screen_size.width,
    };
    let shortcuts_rect = Rect {
        x: 0,
        y: 3,
        height: screen_size.height - 6,
        width: 20,
    };
    let panetab_rect = Rect {
        x: 20,
        y: 3,
        height: 3,
        width: panetab_preferred_width() as u16,
    };
    let musicpane_rect = Rect {
        x: 20,
        y: 6,
        height: screen_size.height
            - (gauge_rect.height + searchbar_rect.height + panetab_rect.height),
        width: screen_size.width - 20,
    };
    let playlistpane_rect = Rect { ..musicpane_rect };
    let artistpane_rect = Rect {
        ..playlistpane_rect
    };
    let musicpane_division = PaneDivision::<3> {
        spacing: 1,
        splits: [
            Constraint::Percentage(60),
            Constraint::Percentage(25),
            Constraint::Percentage(15),
        ],
    };
    let playlistpane_division = PaneDivision::<3> {
        spacing: 1,
        splits: [
            Constraint::Length(5),
            Constraint::Length(playlistpane_rect.width - (5 + 20)),
            Constraint::Length(20),
        ],
    };

    let geometry = GeometryData {
        artistpane: artistpane_rect,
        playlistpane: playlistpane_rect,
        musicpane: musicpane_rect,
        gauge: gauge_rect,
        searchbar: searchbar_rect,
        shortcuts: shortcuts_rect,
        panetab: panetab_rect,
        musicpane_division,
        playlistpane_division,
    };

    let theme = Theme {
        active_color: RGB(10, 150, 150),
        highlight_color: RGB(200, 160, 0),
        base_color: RGB(175, 125, 115),
        inactive_color: RGB(200, 160, 0),
    };

    let appdata = ExampleAppdata::get_filled();

    draw_all_ui(f, &appdata, &theme, geometry)
}
