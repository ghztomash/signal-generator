use ratatui::{
    layout::{Constraint, Direction, Layout, Margin},
    prelude::{Alignment, Frame, Marker},
    style::{Color, Style, Stylize},
    widgets::{
        canvas::*, Axis, Block, BorderType, Borders, Chart, Clear, Dataset, GraphType, Paragraph,
        Sparkline, Tabs, Widget,
    },
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    frame.render_widget(Clear, frame.size());

    let area = frame.size();

    let sub_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(area);

    let title_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(sub_area[0]);

    // main area is split into two, left preview and right control panel
    let main_sub_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(sub_area[1]);

    let tab_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(main_sub_area[1].inner(&Margin {
            horizontal: 1,
            vertical: 1,
        }));

    frame.render_widget(make_tab_bar(app), title_area[1]);

    frame.render_widget(make_preview_canvas(), main_sub_area[0]);

    frame.render_widget(
        Block::default()
            .title("Control Panel")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
        main_sub_area[1],
    );

    frame.render_widget(make_status_bar(), sub_area[2])
}

fn make_tab_bar(app: &mut App) -> impl Widget + 'static {
    let tab_titles = vec!["Channel A", "Channel B", "Output", "Tab4"];
    Tabs::new(tab_titles)
        .style(Style::default())
        .highlight_style(Style::default().fg(Color::Green))
        .select(app.tab_index)
}

fn make_preview_canvas() -> impl Widget + 'static {
    Canvas::default()
        .block(
            Block::default()
                .title("Canvas")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .marker(Marker::Braille)
        .paint(|ctx| {
            ctx.draw(&Map {
                color: Color::Green,
                resolution: MapResolution::High,
            });
            ctx.print(10.0, 10.0, "You are here".yellow());
        })
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
}

fn make_status_bar() -> impl Widget + 'static {
    Paragraph::new(format!("Press `Esc`, `Ctrl-C` or `q` to stop running."))
        .block(
            Block::default()
                .title("Status")
                .title_alignment(Alignment::Left)
                .title_style(Style::default().fg(Color::Green))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Left)
}
