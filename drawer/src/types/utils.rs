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

pub fn geometry_from_rect_map<RectIter>(rect_map: RectIter) -> GeometryData
where
    RectIter: Iterator<Item = (ItemIdentifier, MyRect)>,
{
    let _ = rect_map;
    unimplemented!()
}
