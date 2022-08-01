mod common;
use common::*;
use drawer::gadgets::shortcut;
use tui::layout::Rect;
use tui::widgets::ListState;

struct ExampleShortcutAppdata;

impl shortcut::ShortcutListAppdata for ExampleShortcutAppdata {
    fn is_shortcutlist_active(&self) -> bool {
        true
    }

    fn selected(&self) -> Option<usize> {
        Some(1_usize)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    entrypoint(draw_searchbar)
}

fn draw_searchbar<B: Backend>(f: &mut Frame<B>) {
    let theme = get_default_theme();
    let shortcut = shortcut::get_shortcut_list(ExampleShortcutAppdata, &theme);
    let place = Rect {
        x: 0,
        y: 3,
        height: f.size().height - 10,
        width: 22,
    };

    f.render_widget(shortcut, place);
}
