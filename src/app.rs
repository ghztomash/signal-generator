use waveforms_rs::Waveform;

/// Application state
const TAB_COUNT: usize = 4;
const PARAMETER_COUNT: usize = 5;

#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub command_mode: bool,
    pub command: String,
    pub tab_index: usize,
    pub parameter_index: usize,
    pub parameter: Parameter,
    pub waveform_preview_a: Waveform,
    pub waveform_preview_b: Waveform,
}

#[derive(Default, Debug)]
pub enum Parameter {
    #[default]
    Frequency,
    Amplitude,
    Waveform,
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

    pub fn set_command_mode(&mut self, enabled: bool) {
        self.command_mode = enabled;
    }

    pub fn process_command(&mut self) {
        self.waveform_preview_b
            .set_waveform_type(waveforms_rs::WaveformType::Triangle);
        // self.waveform_preview_b.set_frequency(880.0);
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
