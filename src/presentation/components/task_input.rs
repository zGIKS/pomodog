use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::domain::{App, MAX_TASK_NAME_LEN};

pub fn render(f: &mut Frame, app: &App, area: ratatui::layout::Rect, frame_count: usize) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(10),
            Constraint::Fill(1),
        ])
        .split(area);

    let inner_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(44), // Consistent width with Menu
            Constraint::Fill(1),
        ])
        .split(chunks[1]);

    let config = &app.configs()[app.selected_index()];

    let header = Paragraph::new(vec![
        Line::from(format!(" MODE: {} ", config.label).bold().cyan()),
        Line::from("────────────────────────────────────────".dim()),
        Line::from(" What are you working on? ".white()),
        Line::from(""),
    ])
    .alignment(Alignment::Center);

    let input_text = if app.task_name().is_empty() {
        Span::styled(
            "Type task name...",
            Style::default().fg(Color::DarkGray).italic(),
        )
    } else {
        Span::styled(
            app.task_name().as_str(),
            Style::default().fg(Color::White).bold(),
        )
    };

    let cursor = if frame_count.is_multiple_of(2) {
        "_"
    } else {
        " "
    };

    let input_box = Paragraph::new(Line::from(vec![
        input_text,
        Span::styled(cursor, Style::default().fg(Color::Yellow)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Yellow))
            .title_top(
                Line::from(format!(" {}/{} ", app.task_name().len(), MAX_TASK_NAME_LEN))
                    .right_aligned(),
            ),
    )
    .alignment(Alignment::Center);

    let footer = if let Some(error_msg) = app.get_input_error() {
        Paragraph::new(vec![Line::from(""), Line::from(error_msg.red().bold())])
            .alignment(Alignment::Center)
    } else {
        Paragraph::new(vec![
            Line::from(""),
            Line::from("Enter to Start • Space to Pause/Resume • Backspace to Delete".dim()),
        ])
        .alignment(Alignment::Center)
    };

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(inner_chunks[1]);

    f.render_widget(header, layout[0]);
    f.render_widget(input_box, layout[1]);
    f.render_widget(footer, layout[2]);
}
