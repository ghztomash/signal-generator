use waveforms_rs::Waveform;

/// Application state
const TAB_COUNT: usize = 4;
const PARAMETER_COUNT: usize = 5;

#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub tab_index: usize,
    pub parameter_index: usize,
    pub parameter: Parameter,
    pub mode: Mode,
    pub command: String,
    pub waveform_preview_a: Waveform,
    pub waveform_preview_b: Waveform,
}

#[derive(Default, Debug, PartialEq)]
pub enum Parameter {
    #[default]
    Frequency,
    Amplitude,
    Waveform,
}

#[derive(Default, Debug, PartialEq)]
pub enum Mode {
    #[default]
    Normal,
    Command,
    Keypad,
    Help,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn next_tab(&mut self) {
        self.tab_index = (self.tab_index + 1) % TAB_COUNT;
    }

    pub fn previous_tab(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = TAB_COUNT - 1;
        }
    }

    pub fn set_tab(&mut self, index: usize) {
        if index < TAB_COUNT {
            self.tab_index = index;
        }
    }

    pub fn next_parameter(&mut self) {
        self.parameter_index = (self.parameter_index + 1) % PARAMETER_COUNT;
        self.set_parameter_index();
    }

    pub fn previous_parameter(&mut self) {
        if self.parameter_index > 0 {
            self.parameter_index -= 1;
        } else {
            self.parameter_index = PARAMETER_COUNT - 1;
        }
        self.set_parameter_index();
    }

    pub fn set_parameter_index(&mut self) {
        match self.parameter_index {
            0 => self.parameter = Parameter::Frequency,
            1 => self.parameter = Parameter::Amplitude,
            2 => self.parameter = Parameter::Waveform,
            _ => panic!("wrong parameter index"),
        };
    }

    pub fn increase_parameter_value(&mut self) {
        match self.parameter {
            Parameter::Frequency => {
                let mut frequency = self.waveform_preview_a.frequency();
                frequency += 10.0;
                self.waveform_preview_a.set_frequency(frequency);
            }
            Parameter::Amplitude => {
                let mut amplitude = self.waveform_preview_a.amplitude();
                amplitude += 10.0;
                self.waveform_preview_a.set_amplitude(amplitude);
            }
            Parameter::Waveform => {}
        }
    }

    pub fn set_parameter_value(&mut self, value: f32) {
        match self.parameter {
            Parameter::Frequency => {
                self.waveform_preview_a.set_frequency(value);
            }
            Parameter::Amplitude => {
                self.waveform_preview_a.set_amplitude(value);
            }
            Parameter::Waveform => {}
        }
    }

    pub fn set_normal_mode(&mut self) {
        self.mode = Mode::Normal;
        self.command.clear();
    }

    pub fn set_help_mode(&mut self) {
        self.mode = Mode::Help;
    }

    pub fn set_command_mode(&mut self) {
        self.mode = Mode::Command;
        self.command.clear();
    }

    pub fn push_command_char(&mut self, c: char) {
        match c {
            _ if c.is_alphanumeric() => self.command.push(c),
            ' ' => self.command.push(c),
            ',' => self.command.push(c),
            '.' => self.command.push(c),
            '-' => self.command.push(c),
            '+' => self.command.push(c),
            _ => {}
        }
    }

    pub fn pop_command_char(&mut self) {
        self.command.pop();
    }

    pub fn process_command(&mut self) {
        let command = self.command.to_lowercase();
        self.command.clear();
        let parameters = command.split_whitespace().collect::<Vec<&str>>();

        // process command
        match parameters.first().unwrap_or(&"").as_ref() {
            "q" | "quit" | "exit" => self.quit(),
            "h" | "help" => {
                self.set_help_mode();
                return;
            }
            "f" | "freq" | "frequency" => {
                self.parameter = Parameter::Frequency;
                if parameters.len() > 1 {
                    if let Ok(frequency) = parameters[1].parse::<f32>() {
                        self.set_parameter_value(frequency);
                    }
                }
            }
            "a" | "amp" | "amplitude" => {
                self.parameter = Parameter::Amplitude;
                if parameters.len() > 1 {
                    if let Ok(amplitude) = parameters[1].parse::<f32>() {
                        self.set_parameter_value(amplitude);
                    }
                }
            }
            _ => (),
        }

        // reset normal mode
        self.set_normal_mode();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_quit() {
        let mut app = App::new();
        assert_eq!(app.should_quit, false);
        app.quit();
        assert_eq!(app.should_quit, true);
    }

    #[test]
    fn test_app_next_tab() {
        let mut app = App::new();
        assert_eq!(app.tab_index, 0);
        app.next_tab();
        assert_eq!(app.tab_index, 1);
        app.next_tab();
        assert_eq!(app.tab_index, 2);
        app.next_tab();
        assert_eq!(app.tab_index, 3);
        app.next_tab();
        assert_eq!(app.tab_index, 0);
    }

    #[test]
    fn test_app_previous_tab() {
        let mut app = App::new();
        assert_eq!(app.tab_index, 0);
        app.previous_tab();
        assert_eq!(app.tab_index, 3);
        app.previous_tab();
        assert_eq!(app.tab_index, 2);
        app.previous_tab();
        assert_eq!(app.tab_index, 1);
        app.previous_tab();
        assert_eq!(app.tab_index, 0);
    }

    #[test]
    fn test_app_set_tab() {
        let mut app = App::new();
        assert_eq!(app.tab_index, 0);
        app.set_tab(2);
        assert_eq!(app.tab_index, 2);
        app.set_tab(5);
        assert_eq!(app.tab_index, 2);
    }
}
