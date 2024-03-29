use ratatui::{
    layout::{Constraint, Direction, Layout, Margin},
    prelude::{Alignment, Frame, Marker, Modifier, Rect, Text},
    style::{Color, Style, Stylize},
    symbols,
    widgets::{
        canvas::*, Axis, Block, BorderType, Borders, Cell, Chart, Clear, Dataset, Gauge, GraphType,
        LineGauge, List, ListItem, ListState, Paragraph, Row, StatefulWidget, Table, TableState,
        Tabs, Widget,
    },
};

use crate::app::TAB_TITLES;
use crate::app::{App, Mode};

pub const HELP_LOGO: &str = r#"
  ___ (_)__ ____  ___ _/ /__ ____ ___  ___ _______ _/ /____  ____
 (_-</ / _ `/ _ \/ _ `/ / _ `/ -_) _ \/ -_) __/ _ `/ __/ _ \/ __/
/___/_/\_, /_//_/\_,_/_/\_, /\__/_//_/\__/_/  \_,_/\__/\___/_/   
      /___/            /___/                                     
"#;

/// Help text to show.
pub const HELP_TEXT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    " v",
    env!("CARGO_PKG_VERSION"),
    "\n",
    env!("CARGO_PKG_REPOSITORY"),
    "\nwritten by ",
    env!("CARGO_PKG_AUTHORS"),
);

pub fn render(app: &mut App, frame: &mut Frame) {
    frame.render_widget(Clear, frame.size());

    let area = frame.size().inner(&Margin {
        horizontal: 0,
        vertical: 0,
    });

    let sub_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
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
        .constraints([Constraint::Min(5), Constraint::Length(3)])
        .split(main_sub_area[0]);

    frame.render_widget(
        Paragraph::new(format!("Signal Generator v{}", env!("CARGO_PKG_VERSION")))
            .style(Style::default())
            .alignment(Alignment::Right),
        title_area[1],
    );

    let tab_color = match app.tab_index {
        0 => Color::Yellow,
        1 => Color::Cyan,
        _ => Color::White,
    };

    frame.render_widget(make_tab_bar(app, tab_color), title_area[0]);

    frame.render_widget(make_preview_canvas(app), main_sub_area[1]);

    let waveform = format!(
        "{}",
        app.waveform_previews[app.selected_waveform].waveform_type()
    );
    let frequency = format!(
        "{:.2} Hz",
        app.waveform_previews[app.selected_waveform].frequency()
    );
    let amplitude = format!(
        "{:.2}",
        app.waveform_previews[app.selected_waveform].amplitude()
    );
    let phase_offset = format!(
        "{:.2}",
        app.waveform_previews[app.selected_waveform].phase_offset()
    );
    let dc_offset = format!(
        "{:.2}",
        app.waveform_previews[app.selected_waveform].dc_offset()
    );
    let parameters = vec![
        vec!["Waveform:", waveform.as_str()],
        vec!["Frequency:", frequency.as_str()],
        vec!["Amplitude:", amplitude.as_str()],
        vec!["Phase offset:", phase_offset.as_str()],
        vec!["DC offset:", dc_offset.as_str()],
        vec!["Pan:", "0.0"],
    ];
    frame.render_stateful_widget(
        make_parameter_table(parameters, tab_color),
        tab_area[0],
        &mut app.table_state,
    );

    frame.render_widget(make_status_bar(app), tab_area[1]);

    if app.mode == Mode::Help {
        let area = centered_rect(80, 60, area);
        frame.render_widget(Clear, area); //this clears out the background
        frame.render_widget(make_help_popup(app), area);
    }
}

fn make_parameter_table<'a>(
    parameters: Vec<Vec<&'a str>>,
    tab_color: Color,
) -> impl StatefulWidget<State = TableState> + 'a {
    let selected_style = Style::default()
        .fg(tab_color)
        .add_modifier(Modifier::REVERSED);

    let rows = parameters.iter().map(|item| {
        let cells = item.iter().map(|c| Cell::from(*c));
        Row::new(cells)
    });

    Table::new(
        rows,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(tab_color)),
    )
    .highlight_style(selected_style)
}

fn make_help_popup(app: &App) -> impl Widget + 'static {
    let help_text = Text::raw(format!("{}\n\n{}", HELP_LOGO, HELP_TEXT));
    Paragraph::new(help_text)
        .block(
            Block::default()
                .title(" Help ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Thick),
        )
        .style(Style::default())
        .alignment(Alignment::Left)
}

fn make_tab_bar(app: &mut App, tab_color: Color) -> impl Widget + 'static {
    Tabs::new(TAB_TITLES.to_vec())
        .style(Style::default())
        .highlight_style(
            Style::default()
                .fg(tab_color)
                .add_modifier(Modifier::REVERSED),
        )
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
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
        )
        .marker(Marker::Braille)
        .paint(move |ctx| {
            ctx.draw(&Line {
                x1: 0.0,
                y1: 0.0,
                x2: 100.0,
                y2: 0.0,
                color: Color::DarkGray,
            });
            ctx.print(0.0, -1.0, "-1.0".dark_gray());
            ctx.print(0.0, 0.0, "0.0".dark_gray());
            ctx.print(0.0, 1.0, "+1.0".dark_gray());

            // draw the waveforms
            let mut p1;
            let mut p2;
            for i in 0..values_a.len() - 1 {
                p1 = values_a.get(i).unwrap_or(&(0.0, 0.0));
                p2 = values_a.get(i + 1).unwrap_or(&(0.0, 100.0));
                ctx.draw(&Line {
                    x1: p1.0,
                    y1: p1.1,
                    x2: p2.0,
                    y2: p2.1,
                    color: Color::Yellow,
                });
            }
            for i in 0..values_b.len() - 1 {
                p1 = values_b.get(i).unwrap_or(&(0.0, 0.0));
                p2 = values_b.get(i + 1).unwrap_or(&(0.0, 100.0));
                ctx.draw(&Line {
                    x1: p1.0,
                    y1: p1.1,
                    x2: p2.0,
                    y2: p2.1,
                    color: Color::Cyan,
                });
            }
        })
        .x_bounds([0.0, 100.0])
        .y_bounds([-1.1, 1.1])
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
                // .title("Status")
                // .title_alignment(Alignment::Left)
                // .title_style(Style::default().fg(Color::White))
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::White)),
        )
        .style(Style::default().fg(status_color))
        .alignment(Alignment::Left)
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
