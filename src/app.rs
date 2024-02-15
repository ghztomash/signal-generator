use crate::audio::AudioStream;
use crate::parameter::Parameter;
use ratatui::widgets::TableState;
use std::sync::{Arc, Mutex};
use waveforms_rs::Waveform;

/// Application state
pub const WAVEFORMS_COUNT: usize = 2;
const TAB_COUNT: usize = 3;
pub const TAB_TITLES: [&str; 3] = ["Channel A", "Channel B", "Output"];

#[derive(Default)]
pub struct App {
    pub should_quit: bool,
    pub tab_index: usize,
    pub selected_parameter: Parameter,
    pub table_state: TableState,
    pub mode: Mode,
    // string buffer for command mode
    pub command: String,
    pub command_history: Vec<String>,
    command_history_index: usize,
    // warning message to display
    pub warning: Option<String>,
    // waveform preview generators for each channel
    pub waveform_previews: Vec<Waveform>,
    pub selected_waveform: usize,
    pub audio: AudioStream,
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
        // generate waveform previews
        let mut waveform_previews: Vec<Waveform> = Vec::new();
        for i in 0..WAVEFORMS_COUNT {
            waveform_previews.push(Waveform::new(44100.0, 440.0 * (i as f32 + 1.0)));
        }

        let mut table_state = TableState::default();
        table_state.select(Some(0));

        Self {
            should_quit: false,
            tab_index: 0,
            selected_parameter: Parameter::Frequency,
            mode: Mode::Normal,
            command: String::new(),
            waveform_previews,
            selected_waveform: 0,
            command_history: vec!["".to_string()],
            command_history_index: 0,
            warning: None,
            table_state,
            audio: AudioStream::default(),
        }
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn next_tab(&mut self) {
        self.set_tab((self.tab_index + 1) % TAB_COUNT);
    }

    pub fn previous_tab(&mut self) {
        if self.tab_index > 0 {
            self.set_tab(self.tab_index - 1);
        } else {
            self.set_tab(TAB_COUNT - 1);
        }
    }

    pub fn set_tab(&mut self, index: usize) {
        if index < TAB_COUNT {
            self.tab_index = index;
        }
        if index < self.waveform_previews.len() {
            self.selected_waveform = index;
        }
    }

    pub fn next_parameter(&mut self) {
        self.selected_parameter = self.selected_parameter.next();
        let value = self.selected_parameter as usize;
        self.table_state.select(Some(value));
    }

    pub fn previous_parameter(&mut self) {
        self.selected_parameter = self.selected_parameter.previous();
        let value = self.selected_parameter as usize;
        self.table_state.select(Some(value));
    }

    pub fn increase_parameter_value(&mut self, parameter: Parameter) {
        match parameter {
            Parameter::Frequency => {
                let mut frequency = self.waveform_previews[self.selected_waveform].frequency();
                frequency += 1.0;
                self.waveform_previews[self.selected_waveform].set_frequency(frequency);
            }
            Parameter::Amplitude => {
                let mut amplitude = self.waveform_previews[self.selected_waveform].amplitude();
                amplitude += 1.0;
                self.waveform_previews[self.selected_waveform].set_amplitude(amplitude);
            }
            Parameter::Waveform => {
                let waveform =
                    *self.waveform_previews[self.selected_waveform].waveform_type() as usize;
                if let Ok(wave) = (waveform + 1).try_into() {
                    self.waveform_previews[self.selected_waveform].set_waveform_type(wave);
                }
            }
            _ => {
                self.set_warning("Parameter not implemented");
            }
        }
    }

    pub fn set_parameter_value(&mut self, parameter: Parameter, value: f32) {
        match parameter {
            Parameter::Frequency => {
                self.waveform_previews[self.selected_waveform].set_frequency(value);
                // update audio thread waveforms
                let audio_waveforms = Arc::clone(&self.audio.waveforms);
                let mut waveforms = audio_waveforms.lock().unwrap();
                waveforms[self.selected_waveform].set_frequency(value);
            }
            Parameter::Amplitude => {
                self.waveform_previews[self.selected_waveform].set_amplitude(value);
                // update audio thread waveforms
                let audio_waveforms = Arc::clone(&self.audio.waveforms);
                let mut waveforms = audio_waveforms.lock().unwrap();
                waveforms[self.selected_waveform].set_amplitude(value);
            }
            Parameter::Waveform => {
                if let Ok(waveform) = (value as usize - 1).try_into() {
                    self.waveform_previews[self.selected_waveform].set_waveform_type(waveform);
                    // update audio thread waveforms
                    let audio_waveforms = Arc::clone(&self.audio.waveforms);
                    let mut waveforms = audio_waveforms.lock().unwrap();
                    waveforms[self.selected_waveform].set_waveform_type(waveform);
                }
            }
            _ => {
                self.set_warning("Parameter not implemented");
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
            ' ' | ',' | '.' | '-' | '+' => self.command.push(c),
            _ => {}
        }
    }

    pub fn pop_command_char(&mut self) {
        self.command.pop();
    }

    pub fn process_command(&mut self) {
        let command = self.command.trim().to_lowercase();
        self.command.clear();

        // add to command history
        if command.len() > 0 {
            self.command_history.insert(1, command.clone());
            self.command_history_index = 0;
        }

        // process command
        let parameters = command
            .split_whitespace()
            .filter(|x| x.len() > 0)
            .collect::<Vec<&str>>();
        match parameters.first().unwrap_or(&"").as_ref() {
            "q" | "quit" | "exit" => self.quit(),
            "h" | "help" => {
                self.set_help_mode();
                return;
            }
            "f" | "freq" | "frequency" => {
                if parameters.len() > 1 {
                    if let Ok(frequency) = parameters.get(1).unwrap_or(&"").parse::<f32>() {
                        self.set_parameter_value(Parameter::Frequency, frequency);
                    } else {
                        self.set_warning("Invalid frequency");
                    }
                } else {
                    self.set_warning("No parameter");
                }
            }
            "a" | "amp" | "amplitude" => {
                if parameters.len() > 1 {
                    if let Ok(amplitude) = parameters.get(1).unwrap_or(&"").parse::<f32>() {
                        self.set_parameter_value(Parameter::Amplitude, amplitude);
                    } else {
                        self.set_warning("Invalid amplitude");
                    }
                } else {
                    self.set_warning("No parameter");
                }
            }
            "w" | "wave" | "waveform" => {
                if parameters.len() > 1 {
                    if let Ok(waveform) = parameters.get(1).unwrap_or(&"").parse::<u8>() {
                        self.set_parameter_value(Parameter::Waveform, waveform as f32);
                    } else {
                        match parameters.get(1).unwrap_or(&"").to_lowercase().as_ref() {
                            "sine" => {
                                self.set_parameter_value(Parameter::Waveform, 1.0);
                            }
                            "square" => {
                                self.set_parameter_value(Parameter::Waveform, 2.0);
                            }
                            "triangle" => {
                                self.set_parameter_value(Parameter::Waveform, 3.0);
                            }
                            "sawtooth" => {
                                self.set_parameter_value(Parameter::Waveform, 4.0);
                            }
                            "noise" => {
                                self.set_parameter_value(Parameter::Waveform, 5.0);
                            }
                            _ => {
                                self.set_warning("Invalid waveform");
                            }
                        }
                    }
                } else {
                    self.set_warning("No parameter");
                }
            }
            _ => {
                self.set_warning("Unknown command");
            }
        }

        // reset normal mode
        self.set_normal_mode();
    }

    pub fn command_history_last(&mut self) {
        if self.command_history.len() == 0 {
            return;
        }

        if self.command_history_index < self.command_history.len() - 1 {
            self.command_history_index += 1;
            self.command = self.command_history[self.command_history_index].clone();
        }
    }

    pub fn command_history_next(&mut self) {
        if self.command_history.len() == 0 {
            return;
        }

        if self.command_history_index > 0 {
            self.command_history_index -= 1;
            self.command = self.command_history[self.command_history_index].clone();
        }
    }

    pub fn set_warning(&mut self, warning: &str) {
        self.warning = Some(warning.to_string());
    }

    pub fn clear_warning(&mut self) {
        self.warning = None;
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
