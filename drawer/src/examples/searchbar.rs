mod common;
use common::*;
use drawer::gadgets::searchbar;

struct ExampleSearchbarAppdata;

impl searchbar::SearchbarAppdata for ExampleSearchbarAppdata {
    fn is_searchbar_active(&self) -> bool {
        true
    }

    fn get_altering_query<'a>(&'a self) -> &'a str{
        "example search"
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    entrypoint(ui)
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let searchbar = searchbar::get_searchbar(ExampleSearchbarAppdata);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Max(f.size().height)
            ]
            .as_ref(),
        )
        .split(f.size());

    let block = Block::default().title("Block").borders(Borders::ALL);
    f.render_widget(searchbar, chunks[0]);
}
