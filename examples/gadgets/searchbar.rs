mod common;
use common::*;
use drawer::gadgets::searchbar;
use tui::layout::Rect;

struct ExampleSearchbarAppdata;

impl searchbar::SearchbarAppdata for ExampleSearchbarAppdata {
    fn is_searchbar_active(&self) -> bool {
        true
    }

    fn get_altering_query(&self) -> &str {
        "example search"
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    entrypoint(draw_searchbar)
}

fn draw_searchbar<B: Backend>(f: &mut Frame<B>) {
    let theme = get_default_theme();
    let searchbar = searchbar::get_searchbar(&ExampleSearchbarAppdata, &theme);
    let place = Rect {
        x: 0,
        y: 0,
        height: 3,
        width: f.size().width,
    };

    f.render_widget(searchbar, place);
}
