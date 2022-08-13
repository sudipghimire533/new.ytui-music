use std::collections::HashMap;

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
            .remove(&ItemIdentifier::Custom(name))
            .map(from_my_rect)
    };

    let searchbar = get_rect("serachbar".to_string()).ok_or("Cannot get position for searchbar")?;
    let sidebar = get_rect("serachbar".to_string()).ok_or("Cannot get position for searchbar")?;
    let gauge = get_rect("serachbar".to_string()).ok_or("Cannot get position for searchbar")?;
    let panetab = get_rect("serachbar".to_string()).ok_or("Cannot get position for searchbar")?;
    let [musicpane, playlistpane, artistpane] =
        { [get_rect("serachbar".to_string()).ok_or("Cannot get position for searchbar")?; 3] };

    Ok(GeometryData {
        searchbar,
        sidebar,
        gauge,
        panetab,
        musicpane,
        playlistpane,
        artistpane,
        musicpane_division: todo!(),
        playlistpane_division: todo!(),
    })
}
