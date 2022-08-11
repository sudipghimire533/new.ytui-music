use std::error::Error;
use std::io;
use std::io::Stdout;
use tui::Terminal;

#[cfg(all(feature = "crossterm", not(feature = "termion")))]
pub use init_crossterm::setup_terminal;

#[cfg(all(feature = "termion", not(feature = "crossterm")))]
pub use init_termion::setup_terminal;

#[cfg(all(feature = "termion", feature = "crossterm"))]
pub fn setup_terminal() -> ! {
    panic!("Enable either termion or crossterm. Cannot proceed with both enabled");
}

#[cfg(not(any(feature = "crossterm", feature = "termion")))]
pub fn setup_terminal() -> ! {
    panic!("No backend feature to handle initilization terminal")
}

#[cfg(feature = "termion")]
pub mod init_termion {
    use super::*;
    use tui::backend::TermionBackend;

    pub fn setup_terminal() -> Result<
        (
            Terminal<TermionBackend<Stdout>>,
            impl FnOnce(&mut Terminal<TermionBackend<Stdout>>) -> Result<(), Box<dyn Error>>,
        ),
        Box<dyn Error>,
    > {
        if false {
            Ok((unimplemented!(), rollback_terminal))
        } else {
            Err("Termion backend to setup terminal is not yet implemented")?
        }
    }

    pub fn rollback_terminal(
        terminal: &mut Terminal<TermionBackend<Stdout>>,
    ) -> Result<(), Box<dyn Error>> {
        let _ = terminal;
        Err("Terminal backend to rollback terminal is not yet implemented")?
    }
}

#[cfg(feature = "crossterm")]
pub mod init_crossterm {
    use super::*;
    use crossterm::execute;
    use crossterm::terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    };
    use tui::backend::CrosstermBackend;

    pub fn setup_terminal() -> Result<
        (
            // the terminal handle itself
            Terminal<CrosstermBackend<Stdout>>,
            // also the function pointer to
            // tollback this setup
            impl FnOnce(&mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>>,
        ),
        Box<dyn Error>,
    > {
        enable_raw_mode()?;

        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        terminal.hide_cursor()?;

        Ok((terminal, rollback_terminal))
    }

    pub fn rollback_terminal(
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), Box<dyn Error>> {
        disable_raw_mode()?;

        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        Ok(())
    }
}
