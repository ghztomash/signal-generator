use crate::app::{App, Mode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn update(app: &mut App, key_event: KeyEvent) {
    // ignore key repeat events
    if key_event.kind == crossterm::event::KeyEventKind::Repeat {
        return;
    }

    app.clear_warning();

    if key_event.code == KeyCode::Esc {
        app.set_normal_mode();
        return;
    } else if app.mode == Mode::Command {
        match key_event.code {
            KeyCode::Char(c) => {
                app.push_command_char(c);
            }
            KeyCode::Backspace => {
                app.pop_command_char();
            }
            KeyCode::Up => {
                app.command_history_last();
            }
            KeyCode::Down => {
                app.command_history_next();
            }
            KeyCode::Enter => {
                app.process_command();
            }
            _ => {}
        }
        return;
    }
    // normal mode
    match key_event.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            if app.mode == Mode::Normal {
                app.quit();
            } else {
                app.set_normal_mode();
            }
        }
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Char('h') | KeyCode::Char('H') => {
            if app.mode == Mode::Help {
                app.set_normal_mode();
            } else {
                app.set_help_mode();
            }
        }
        // parameter shortcuts
        KeyCode::Char('f') | KeyCode::Char('F') => {
            app.set_command_mode();
            app.command.push_str("freq ");
        }
        KeyCode::Char('a') | KeyCode::Char('A') => {
            app.set_command_mode();
            app.command.push_str("amp ");
        }
        KeyCode::Char('w') | KeyCode::Char('W') => {
            app.set_command_mode();
            app.command.push_str("wave ");
        }

        KeyCode::Tab => app.next_tab(),
        KeyCode::BackTab => app.previous_tab(),

        KeyCode::Char(':') => app.set_command_mode(),
        KeyCode::Char('1') => app.set_tab(0),
        KeyCode::Char('2') => app.set_tab(1),
        _ => {}
    };
}
