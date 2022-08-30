pub use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
pub use std::{error::Error, io};
pub use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame, Terminal,
};
pub use user_config::preferences::theme::Theme;
pub use user_config::styles::color::RGB;

pub fn get_default_theme() -> Theme {
    Theme {
        active_color: RGB(10, 150, 150),
        highlight_color: RGB(200, 160, 0),
        base_color: RGB(175, 125, 115),
        inactive_color: RGB(200, 160, 0),
    }
}

pub fn entrypoint(
    run: impl Fn(&mut tui::terminal::Frame<CrosstermBackend<std::io::Stdout>>),
) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal, run);

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

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    updater: impl Fn(&mut tui::terminal::Frame<B>),
) -> io::Result<()> {
    loop {
        terminal.draw(|f| updater(f))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}
