use waveforms_rs::Waveform;

/// Application state
const TAB_COUNT: usize = 4;

#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub tab_index: usize,
    pub waveform_preview: Waveform,
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
