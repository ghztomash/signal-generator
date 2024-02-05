use crate::app::{App, Mode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn update(app: &mut App, key_event: KeyEvent) {
    // ignore key repeat events
    if key_event.kind == crossterm::event::KeyEventKind::Repeat {
        return;
    }

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
            app.set_help_mode();
        }
        KeyCode::Tab => app.next_tab(),
        KeyCode::BackTab => app.previous_tab(),

        KeyCode::Char(':') => app.set_command_mode(),
        KeyCode::Char('1') => app.set_tab(0),
        KeyCode::Char('2') => app.set_tab(1),
        KeyCode::Char('3') => app.set_tab(2),
        KeyCode::Char('4') => app.set_tab(3),
        _ => {}
    };
}
