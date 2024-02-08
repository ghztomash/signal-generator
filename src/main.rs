mod app;
mod event;
mod parameter;
mod tui;
mod ui;
mod update;

use app::App;
use color_eyre::eyre::Result;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use update::update;

fn main() -> Result<()> {
    color_eyre::install()?;
    // Create apllication instance
    let mut app = App::new();

    // Initialize terminal
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(125);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start the main loop
    while !app.should_quit {
        // Render the UI
        tui.draw(&mut app)?;

        // Handle events
        match tui.event_handler.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    // Exit the terminal
    tui.exit()?;
    Ok(())
}
