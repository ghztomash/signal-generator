use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(format!("Press `Esc`, `Ctrl-C` or `q` to stop running."))
            .block(
                Block::default()
                    .title("Signal Generator")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center),
        frame.size(),
    )
}
