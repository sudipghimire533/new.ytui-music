use crate::gadgets::AppData;
use std::borrow::Cow;
use tui::widgets::{Paragraph, Block, Borders};
use tui::text::{Span, Spans, Text};
use crate::gadgets::Window;
use tui::style::Style;

pub trait SearchbarAppdata {
    fn is_searchbar_active(&self) -> bool;
    fn get_altering_query<'a>(&'a self) -> &'a str;
}

impl SearchbarAppdata for AppData {
    fn is_searchbar_active(&self) -> bool {
        self.active_window == Window::SearchBar
    }

    fn get_altering_query<'a>(&'a self) -> &'a str {
        &self.altering_query
    }
}

pub fn get_searchbar<'a, 'b, 'c>(
    appdata: impl SearchbarAppdata + 'c,
) -> Paragraph<'c> {
    let text: Text = Spans::from(vec![
        Span::styled(appdata.get_altering_query().to_string(), Default::default()),
        Span::styled(Cow::Borrowed("/"), Default::default())
    ]).into();

    let block_title: Span;
    let border_style: Style; // make this to reference one of already defined active/inactive border_style
    if appdata.is_searchbar_active() {
        block_title = Span::styled("Search ", Default::default());
        border_style = Default::default();
    } else {
        block_title = Span::styled("Search ", Default::default());
        border_style = Default::default();
    }

    let block = Block::default()
        .border_type(tui::widgets::BorderType::Rounded)
        .title(block_title)
        .borders(Borders::ALL)
        .border_style(border_style);
    
    Paragraph::new(text)
        .block(block)
}
