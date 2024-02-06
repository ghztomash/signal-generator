use ratatui::{
    layout::{Constraint, Direction, Layout, Margin},
    prelude::{Alignment, Frame, Marker, Rect},
    style::{Color, Style, Stylize},
    symbols,
    widgets::{
        canvas::*, Axis, Block, BorderType, Borders, Chart, Clear, Dataset, Gauge, GraphType,
        LineGauge, Paragraph, Tabs, Widget,
    },
};

use crate::app::{App, Mode};

pub fn render(app: &mut App, frame: &mut Frame) {
    frame.render_widget(Clear, frame.size());

    let area = frame.size().inner(&Margin {
        horizontal: 2,
        vertical: 1,
    });

    let sub_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(3),
            Constraint::Length(1),
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
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(main_sub_area[0].inner(&Margin {
            horizontal: 1,
            vertical: 1,
        }));

    frame.render_widget(
        Paragraph::new("Signal Generator v{}")
            .style(Style::default())
            .alignment(Alignment::Right),
        title_area[1],
    );

    frame.render_widget(make_tab_bar(app), title_area[0]);

    frame.render_widget(make_preview_canvas(app), main_sub_area[1]);

    frame.render_widget(
        Block::default()
            .title("Control Panel")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
        main_sub_area[0],
    );

    frame.render_widget(
        Paragraph::new("Frequency:")
            .style(Style::default())
            .alignment(Alignment::Left),
        tab_area[0],
    );

    frame.render_widget(make_line_gauge(25.00), tab_area[1]);
    frame.render_widget(make_gauge(25.00), tab_area[2]);

    frame.render_widget(make_status_bar(app), sub_area[2]);

    if app.mode == Mode::Help {
        let block =
            Paragraph::new("Help").block(Block::default().title("Popup").borders(Borders::ALL));
        let area = centered_rect(80, 60, area);
        frame.render_widget(Clear, area); //this clears out the background
        frame.render_widget(block, area);
    }
}

fn make_tab_bar(app: &mut App) -> impl Widget + 'static {
    let tab_titles = vec!["Channel A", "Channel B", "Output"];
    Tabs::new(tab_titles)
        .style(Style::default())
        .highlight_style(Style::default().fg(Color::Green))
        .select(app.tab_index)
}

fn make_preview_canvas(app: &mut App) -> impl Widget + 'static {
    for waveform in app.waveform_previews.iter_mut() {
        waveform.reset();
    }
    let mut values_a: Vec<(f64, f64)> = Vec::new();
    let mut values_b: Vec<(f64, f64)> = Vec::new();
    let mut val;
    for i in 0..100 {
        val = app.waveform_previews[0].process() as f64;
        values_a.push((i as f64, val));
        val = app.waveform_previews[1].process() as f64;
        values_b.push((i as f64, val));
    }
    Canvas::default()
        .block(
            Block::default()
                .title("Canvas")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .marker(Marker::Braille)
        .paint(move |ctx| {
            ctx.draw(&Points {
                coords: &values_a,
                color: Color::Red,
            });
            ctx.draw(&Points {
                coords: &values_b,
                color: Color::Yellow,
            });
            ctx.draw(&Line {
                x1: 0.0,
                y1: 0.0,
                x2: 100.0,
                y2: 0.0,
                color: Color::Blue,
            });
            ctx.print(0.0, -1.0, "-1".gray());
            ctx.print(0.0, 0.0, "0".gray());
            ctx.print(0.0, 1.0, "1".gray());
        })
        .x_bounds([0.0, 100.0])
        .y_bounds([-1.0, 1.0])
}

fn make_status_bar(app: &App) -> impl Widget + 'static {
    let mut status_text = format!("{:?}", app.mode);
    status_text += &" | Press 'h' for help, 'q' to quit.";

    if app.mode == Mode::Command {
        status_text = format!(":{}", app.command);
    }

    let mut status_color = Color::White;

    if app.warning != None {
        status_text = format!("{}!", app.warning.as_ref().unwrap());
        status_color = Color::Red;
    }

    Paragraph::new(status_text)
        .block(
            Block::default()
                .title("Status")
                .title_alignment(Alignment::Left)
                .title_style(Style::default().fg(Color::White))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(status_color))
        .alignment(Alignment::Left)
}

fn make_line_gauge(percent: f64) -> impl Widget + 'static {
    let label = if percent < 100.0 {
        format!("Downloading: {}%", percent)
    } else {
        "Download Complete!".into()
    };
    LineGauge::default()
        .ratio(percent / 100.0)
        .label(label)
        .style(Style::new().light_blue())
        .gauge_style(Style::new().fg(Color::Red))
        .line_set(symbols::line::THICK)
}

fn make_gauge(percent: f64) -> impl Widget + 'static {
    let label = { format!("Amplitude: {}%", percent) };
    Gauge::default()
        .ratio(percent / 100.0)
        .label(label)
        .style(Style::new().light_blue())
        .gauge_style(Style::new().fg(Color::Red))
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
