use waveforms_rs::Waveform;

/// Application state
const TAB_COUNT: usize = 4;

#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub tab_index: usize,
    pub selected_parameter: Parameter,
    pub mode: Mode,
    pub command: String,
    pub waveform_preview_a: Waveform,
    pub waveform_preview_b: Waveform,
}

#[derive(Default, Debug, PartialEq, PartialOrd)]
pub enum Parameter {
    #[default]
    Frequency,
    Amplitude,
    Waveform,
}

impl TryFrom<u8> for Parameter {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Parameter::Frequency),
            1 => Ok(Parameter::Amplitude),
            2 => Ok(Parameter::Waveform),
            _ => Err(()),
        }
    }
}

impl Parameter {
    fn next(&self) -> Self {
        match self {
            Parameter::Frequency => Parameter::Amplitude,
            Parameter::Amplitude => Parameter::Waveform,
            Parameter::Waveform => Parameter::Frequency,
        }
    }

    fn previous(&self) -> Self {
        match self {
            Parameter::Frequency => Parameter::Waveform,
            Parameter::Amplitude => Parameter::Frequency,
            Parameter::Waveform => Parameter::Amplitude,
        }
    }
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
        self.selected_parameter = self.selected_parameter.next();
    }

    pub fn previous_parameter(&mut self) {
        self.selected_parameter = self.selected_parameter.previous();
    }

    pub fn increase_parameter_value(&mut self, parameter: Parameter) {
        match parameter {
            Parameter::Frequency => {
                let mut frequency = self.waveform_preview_a.frequency();
                frequency += 1.0;
                self.waveform_preview_a.set_frequency(frequency);
            }
            Parameter::Amplitude => {
                let mut amplitude = self.waveform_preview_a.amplitude();
                amplitude += 1.0;
                self.waveform_preview_a.set_amplitude(amplitude);
            }
            Parameter::Waveform => {
                let waveform = *self.waveform_preview_a.waveform_type() as u8;
                if let Ok(wave) = (waveform + 1).try_into() {
                self.waveform_preview_a.set_waveform_type(
                    wave
                );}
            }
        }
    }

    pub fn set_parameter_value(&mut self, parameter: Parameter, value: f32) {
        match parameter {
            Parameter::Frequency => {
                self.waveform_preview_a.set_frequency(value);
            }
            Parameter::Amplitude => {
                self.waveform_preview_a.set_amplitude(value);
            }
            Parameter::Waveform => {
                if let Ok(waveform) = (value as u8).try_into() {
                    self.waveform_preview_a.set_waveform_type(waveform);
                }
            }
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
