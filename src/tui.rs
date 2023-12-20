use std::{io, panic};

use color_eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

use crate::{app::App, event::EventHandler, ui};

/// Terminal user interface.
pub struct Tui {
    /// Terminal user interface.
    terminal: CrosstermTerminal,
    /// Terminal event handler.
    pub event_handler: EventHandler,
}

impl Tui {
    /// Create a new terminal user interface.
    pub fn new(terminal: CrosstermTerminal, event_handler: EventHandler) -> Self {
        Self {
            terminal,
            event_handler,
        }
    }

    /// Initialize the terminal user interface.
    pub fn enter(&mut self) -> Result<()> {
        // enter raw mode
        terminal::enable_raw_mode()?;
        // enter alternate screen
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        // Define custom panic hook to exit alternate screen
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("unable to reset terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;

        Ok(())
    }

    /// Reset the terminal user interface.
    pub fn reset() -> Result<()> {
        // exit alternate screen
        crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        // exit raw mode
        terminal::disable_raw_mode()?;
        Ok(())
    }

    /// Exit the terminal user interface.
    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        // self.terminal.clear()?;
        Ok(())
    }

    /// Draw the terminal user interface.
    pub fn draw(&mut self, app: &mut App) -> Result<()> {
        self.terminal.draw(|frame| ui::render(app, frame))?;
        Ok(())
    }
}
