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
        Paragraph::new(format!("Signal Generator v{}", env!("CARGO_PKG_VERSION")))
            .style(Style::default())
            .alignment(Alignment::Right),
        title_area[1],
    );

    frame.render_widget(make_tab_bar(app), title_area[0]);

    frame.render_widget(make_preview_canvas(app), main_sub_area[1]);

    let waveform = format!(
        "Waveform: {}",
        app.waveform_previews[app.selected_waveform].waveform_type()
    );
    let frequency = format!(
        "Frequency: {:.2} Hz",
        app.waveform_previews[app.selected_waveform].frequency()
    );
    let amplitude = format!(
        "Amplitude: {:.2}",
        app.waveform_previews[app.selected_waveform].amplitude()
    );
    let phase_offset = format!(
        "Phase offset: {:.2}",
        app.waveform_previews[app.selected_waveform].phase_offset()
    );
    let dc_offset = format!(
        "DC offset: {:.2}",
        app.waveform_previews[app.selected_waveform].dc_offset()
    );
    let parameters = vec![
        waveform.as_str(),
        frequency.as_str(),
        amplitude.as_str(),
        phase_offset.as_str(),
        dc_offset.as_str(),
    ];
    frame.render_stateful_widget(
        make_parameter_list(parameters),
        tab_area[0],
        &mut app.list_state,
    );

    // frame.render_widget(make_line_gauge(25.00), tab_area[1]);
    // frame.render_widget(make_gauge(25.00), tab_area[2]);

    frame.render_widget(make_status_bar(app), tab_area[1]);

    if app.mode == Mode::Help {
        let area = centered_rect(80, 60, area);
        frame.render_widget(Clear, area); //this clears out the background
        frame.render_widget(make_help_popup(app), area);
    }
}

fn make_parameter_list<'a>(
    parameters: Vec<&'a str>,
) -> impl StatefulWidget<State = ListState> + 'a {
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default();

    let items = parameters
        .iter()
        .map(|item| ListItem::new(*item).style(normal_style));

    List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
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

fn make_tab_bar(app: &mut App) -> impl Widget + 'static {
    Tabs::new(TAB_TITLES.to_vec())
        .style(Style::default())
        .highlight_style(
            Style::default()
                .fg(Color::White)
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
                .title("Canvas")
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
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
                .border_type(BorderType::Plain),
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
