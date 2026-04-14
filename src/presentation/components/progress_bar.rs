use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::domain::{App, Phase};

pub fn render(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);

    let timer_text = format!("   {}   ", app.timer().formatted_time());
    let timer_widget =
        Paragraph::new(timer_text.bold().white().on_dark_gray()).alignment(Alignment::Center);
    f.render_widget(timer_widget, chunks[0]);

    let ratio = app.timer().progress_ratio();
    let percentage = (ratio * 100.0) as u16;
    let width = 40;
    let filled_count = (ratio * width as f32) as usize;

    let mut bar_spans = Vec::new();
    for i in 0..width {
        if i < filled_count {
            let r = (80.0 + (i as f32 / width as f32) * 150.0) as u8;
            let g = (200.0 - (i as f32 / width as f32) * 100.0) as u8;
            let b = 255;
            bar_spans.push(Span::styled("█", Style::default().fg(Color::Rgb(r, g, b))));
        } else {
            bar_spans.push(Span::styled(
                "·",
                Style::default().fg(Color::Rgb(40, 40, 40)),
            ));
        }
    }

    let (status_label, status_color) = match app.phase() {
        Phase::Work => (" WORKING ", Color::Blue),
        Phase::Break => (" RESTING ", Color::Green),
    };

    let label_line = Line::from(vec![
        Span::styled(
            status_label,
            Style::default().bg(status_color).fg(Color::Black).bold(),
        ),
        Span::raw(format!(" {:3}% ", percentage)).dim(),
    ])
    .alignment(Alignment::Center);

    let bar_line = Line::from(bar_spans).alignment(Alignment::Center);

    f.render_widget(
        Paragraph::new(label_line).alignment(Alignment::Center),
        chunks[2],
    );
    f.render_widget(
        Paragraph::new(bar_line).alignment(Alignment::Center),
        chunks[3],
    );
}
