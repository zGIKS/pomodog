use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::domain::App;

pub fn render(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(12),
            Constraint::Fill(1),
        ])
        .split(area);

    let inner_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(40),
            Constraint::Fill(1),
        ])
        .split(chunks[1]);

    let header = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(" ⬢ ", Style::default().fg(Color::Cyan)),
            Span::styled("POMDOG SESSION", Style::default().bold().white()),
            Span::styled(" ⬢ ", Style::default().fg(Color::Cyan)),
        ]),
        Line::from("────────────────────────────".dim()),
    ])
    .alignment(Alignment::Center);

    let mut options = Vec::new();
    for (i, config) in app.configs().iter().enumerate() {
        if i == app.selected_index() {
            options.push(Line::from(vec![
                Span::styled("  » ", Style::default().fg(Color::Yellow).bold()),
                Span::styled(&config.label, Style::default().fg(Color::White).bold()),
                Span::styled(" «", Style::default().fg(Color::Yellow).bold()),
            ]));
        } else {
            options.push(Line::from(vec![
                Span::styled("    ", Style::default()),
                Span::styled(&config.label, Style::default().fg(Color::DarkGray)),
            ]));
        }
    }

    let menu_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan))
        .title(" Select Mode ")
        .title_alignment(Alignment::Center);

    let menu_widget = Paragraph::new(options)
        .block(menu_block)
        .alignment(Alignment::Center);

    let footer = Paragraph::new(vec![
        Line::from(""),
        Line::from("▲▼ Move • Enter Start".dim()),
    ])
    .alignment(Alignment::Center);

    let list_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(5),
            Constraint::Length(3),
        ])
        .split(inner_chunks[1]);

    f.render_widget(header, list_layout[0]);
    f.render_widget(menu_widget, list_layout[1]);
    f.render_widget(footer, list_layout[2]);
}
