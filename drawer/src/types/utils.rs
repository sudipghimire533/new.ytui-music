use std::collections::HashMap;

use tui::layout::Constraint;
use tui::layout::Rect as TuiRect;
use user_config::reexports::Identifier as ItemIdentifier;
use user_config::reexports::Rect as MyRect;

use super::state::GeometryData;

pub fn from_my_rect(my_rect: MyRect) -> TuiRect {
    let MyRect {
        x,
        y,
        height,
        width,
    } = my_rect;
    TuiRect {
        x,
        y,
        height,
        width,
    }
}

pub fn into_my_rect(tui_rect: TuiRect) -> MyRect {
    let TuiRect {
        x,
        y,
        width,
        height,
    } = tui_rect;
    MyRect {
        x,
        y,
        height,
        width,
    }
}

pub fn consume_and_get_geometry(
    rect_map: &mut HashMap<ItemIdentifier, MyRect>,
) -> Result<GeometryData, &'static str> {
    let mut get_rect = |name: String| {
        rect_map
            .remove(&ItemIdentifier::Container(name))
            .map(from_my_rect)
    };

    let searchbar = get_rect("searchbar".to_string()).ok_or("Cannot get position for searchbar")?;
    let sidebar = get_rect("sidebar".to_string()).ok_or("Cannot get position for sidebar")?;
    let gauge = get_rect("gauge".to_string()).ok_or("Cannot get position for gauge")?;
    let panetab = get_rect("panetab".to_string()).ok_or("Cannot get position for panetab")?;
    let [musicpane, playlistpane, artistpane] = {
        [get_rect("result_pane".to_string())
            .ok_or("Cannot get position for music/playlist/artist pane")?; 3]
    };

    // At the end we will also destory any other remaining element
    // this will mostly be container type
    rect_map.drain();

    let musicpane_division = super::state::PaneDivision {
        spacing: 0,
        splits: [Constraint::Length(0); 3],
    };
    let playlistpane_division = super::state::PaneDivision {
        spacing: 0,
        splits: [Constraint::Length(0); 3],
    };

    Ok(GeometryData {
        searchbar,
        sidebar,
        gauge,
        panetab,
        musicpane,
        playlistpane,
        artistpane,
        musicpane_division,
        playlistpane_division,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gadgets::state::PaneDivision;
    use tui::layout::Constraint;

    fn my_rect_with_x(x: u16) -> MyRect {
        MyRect {
            x,
            ..Default::default()
        }
    }

    fn tui_rect_with_x(x: u16) -> TuiRect {
        TuiRect {
            x,
            ..Default::default()
        }
    }

    #[test]
    fn check_consume_and_get_geometry() {
        let mut map = [
            ("searchbar", my_rect_with_x(1)),
            ("sidebar", my_rect_with_x(2)),
            ("gauge", my_rect_with_x(3)),
            ("panetab", my_rect_with_x(4)),
            ("result_pane", my_rect_with_x(5)),
            ("_this-will-not-recognized_", Default::default()),
            ("-something-null-", Default::default()),
        ]
        .into_iter()
        .map(|(identifier, rect)| (ItemIdentifier::Container(identifier.to_string()), rect))
        .collect::<HashMap<_, _>>();

        let result_geometry = consume_and_get_geometry(&mut map);

        let expected_geometry_data = GeometryData {
            searchbar: tui_rect_with_x(1),
            sidebar: tui_rect_with_x(2),
            gauge: tui_rect_with_x(3),
            panetab: tui_rect_with_x(4),
            musicpane: tui_rect_with_x(5),
            playlistpane: tui_rect_with_x(5),
            artistpane: tui_rect_with_x(5),
            musicpane_division: PaneDivision {
                spacing: 0,
                splits: [Constraint::Length(0); 3],
            },
            playlistpane_division: PaneDivision {
                spacing: 0,
                splits: [Constraint::Length(0); 3],
            },
        };

        assert_eq!(Ok(expected_geometry_data), result_geometry);
        assert_eq!(0, map.iter().len());
    }
}
