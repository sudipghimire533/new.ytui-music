use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use layout_config::ui::UI;
use std::collections::hash_map::HashMap;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
    Frame, Terminal,
};

fn into_my_rect(tuis: tui::layout::Rect) -> layout::rect::Rect {
    layout::rect::Rect {
        x: tuis.x,
        y: tuis.y,
        height: tuis.height,
        width: tuis.width,
    }
}

fn from_my_rect(my: layout::rect::Rect) -> tui::layout::Rect {
    tui::layout::Rect {
        x: my.x,
        y: my.y,
        width: my.width,
        height: my.height,
    }
}

fn get_block<'a>(title: String) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .title(title)
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    let terminal_size = f.size();


    let json_str = include_str!("../../layout-config/layout.json");
    let ui: UI = serde_json::from_str(json_str).unwrap();
    let mut size_map = HashMap::new();
    layout::rect_computation::compute_rect_for_item_tree(&ui.item_root, &mut size_map, &into_my_rect(terminal_size));

    for (item, rect) in size_map.iter().filter(|(item, _)| item.is_reserved()) {
        let item_block = get_block(item.clone().into());
        f.render_widget(item_block, from_my_rect(rect.clone()) );
    }
}
