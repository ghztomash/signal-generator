use crate::app::{App, Mode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn update(app: &mut App, key_event: KeyEvent) {
    if key_event.code == KeyCode::Esc {
        app.set_normal_mode();
        return;
    } else if app.mode == Mode::Command {
        // app.push_command_char(key_event.code);
    }
    match key_event.code {
        KeyCode::Esc => {
            app.set_normal_mode();
        }
        KeyCode::Char('q') => {
            if app.mode == Mode::Normal {
                app.quit();
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
        KeyCode::Enter => {
            app.process_command();
            app.set_normal_mode();
        }
        KeyCode::Backspace => {
            // if app.command_mode() {
            //     app.pop_command_char();
            // }
        }
        KeyCode::Char(':') => app.set_command_mode(),
        KeyCode::Char('1') => app.set_tab(0),
        KeyCode::Char('2') => app.set_tab(1),
        KeyCode::Char('3') => app.set_tab(2),
        KeyCode::Char('4') => app.set_tab(3),
        _ => {}
    };
}
