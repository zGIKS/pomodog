use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::domain::{App, AppState, Phase};

const BAR_WIDTH: usize = 40;

fn status_info(app: &App, phase: Phase) -> (&'static str, Color) {
    match (app.state(), phase) {
        (AppState::Paused, _) => (" PAUSED ", Color::Yellow),
        (_, Phase::Work) => (" WORKING ", Color::Blue),
        (_, Phase::Break) => (" RESTING ", Color::Green),
    }
}

fn build_bar_spans(filled: usize) -> Vec<Span<'static>> {
    (0..BAR_WIDTH)
        .map(|i| {
            let style = if i < filled {
                let t = i as f32 / BAR_WIDTH as f32;
                Style::default().fg(Color::Rgb(
                    (80.0 + t * 150.0) as u8,
                    (200.0 - t * 100.0) as u8,
                    255,
                ))
            } else {
                Style::default().fg(Color::Rgb(40, 40, 40))
            };
            Span::styled(if i < filled { "█" } else { "·" }, style)
        })
        .collect()
}

pub fn render(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let session = match app.session() {
        Some(s) => s,
        None => return,
    };

    let chunks = Layout::default()
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);

    let timer_text = format!("   {}   ", session.timer.formatted_time());
    f.render_widget(
        Paragraph::new(timer_text.bold().white().on_dark_gray()).alignment(Alignment::Center),
        chunks[0],
    );

    let ratio = session.timer.progress_ratio();
    let percentage = (ratio * 100.0) as u16;
    let filled = (ratio * BAR_WIDTH as f32) as usize;

    let (label, color) = status_info(app, session.phase);
    let label_line = Line::from(vec![
        Span::styled(label, Style::default().bg(color).fg(Color::Black).bold()),
        Span::raw(format!(" {:3}% ", percentage)).dim(),
    ]);
    f.render_widget(
        Paragraph::new(label_line).alignment(Alignment::Center),
        chunks[2],
    );

    let bar_line = Line::from(build_bar_spans(filled));
    f.render_widget(
        Paragraph::new(bar_line).alignment(Alignment::Center),
        chunks[4],
    );
}
