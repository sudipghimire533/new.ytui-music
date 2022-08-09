mod common;
use common::*;
use drawer::gadgets::panetab;
use tui::layout::Rect;

struct ExamplepanetabAppdata;

impl panetab::PanetabAppdata for ExamplepanetabAppdata {
    fn is_panetab_active(&self) -> bool {
        true
    }

    fn selected(&self) -> usize {
        0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    entrypoint(draw_searchbar)
}

fn draw_searchbar<B: Backend>(f: &mut Frame<B>) {
    let theme = get_default_theme();
    let panetab = panetab::get_panetab(&ExamplepanetabAppdata, &theme);
    let place = Rect {
        x: 20,
        y: 10,
        height: 3,
        width: f.size().width - 40,
    };

    f.render_widget(panetab, place);
}
