use ratatui::{
    layout::{Constraint, Direction, Layout, Margin},
    prelude::{Alignment, Frame, Marker},
    style::{Color, Style, Stylize},
    widgets::{
        Axis, Block, BorderType, Borders, Chart, Clear, Dataset, GraphType, Paragraph, Sparkline,
        Tabs,
    },
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    frame.render_widget(Clear, frame.size());

    let block = Block::default()
        .title("Signal Generator")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let area = frame.size();

    frame.render_widget(block, area);

    let sub_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(area.inner(&Margin {
            horizontal: 1,
            vertical: 1,
        }));

    let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"])
        //.block(Block::default().title("Tabs").borders(Borders::ALL))
        .style(Style::default())
        .highlight_style(Style::default().fg(Color::Green))
        .select(0);
    frame.render_widget(tabs, sub_area[0]);

    let main_sub_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(sub_area[1]);

    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .title("Sparkline")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .data(&[0, 2, 3, 4, 1, 4, 10, 4, 3, 2, 4, 5, 6, 7, 8, 9, 10])
        .max(10)
        .style(Style::default().red().on_white());
    frame.render_widget(sparkline, main_sub_area[0]);

    frame.render_widget(make_chart(), main_sub_area[1]);

    frame.render_widget(
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
            .alignment(Alignment::Left),
        sub_area[2],
    )
}

fn make_chart() -> Chart<'static> {
    // Create the datasets to fill the chart with
    let datasets = vec![
        // Scatter chart
        Dataset::default()
            .name("data1")
            .marker(Marker::Dot)
            .graph_type(GraphType::Line)
            .style(Style::default().cyan())
            .data(&[(0.0, 5.0), (1.0, 6.0), (1.5, 6.434)]),
        // Line chart
        Dataset::default()
            .name("data2")
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().magenta())
            .data(&[(4.0, 5.0), (5.0, 8.0), (7.66, 13.5)]),
    ];

    // Create the X axis and define its properties
    let x_axis = Axis::default().bounds([0.0, 10.0]);

    // Create the Y axis and define its properties
    let y_axis = Axis::default().bounds([0.0, 10.0]);

    // Create the chart and link all the parts together
    let chart = Chart::new(datasets)
        .block(Block::default().title("Chart"))
        .x_axis(x_axis)
        .y_axis(y_axis);

    chart
}
