use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Stylize,
    widgets::Paragraph,
    Frame,
};

use super::speech_bubble;
use crate::domain::{App, AppState, Phase};

pub const DOG_FRAME_1: &str = r#"      .─.        
     { }``;      
     / ( '       
  (  /   |        
   \(_)_]]        "#;

pub const DOG_FRAME_2: &str = r#"      .─.        
     [ ]``;      
     / ( '       
   ) /   |        
   \(_)_]]        "#;

pub fn render(f: &mut Frame, app: &App, area: ratatui::layout::Rect, frame_count: usize) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Length(6)])
        .split(area);

    let message = if let Some(session) = app.session() {
        match (app.state(), session.phase) {
            (AppState::Paused, _) => String::from("PAUSED..."),
            (_, Phase::Work) => {
                if session.task_name.is_empty() {
                    String::from("WORKING...")
                } else {
                    format!("FOCUSING ON: {}", session.task_name.as_str())
                }
            }
            (_, Phase::Break) => {
                if session.task_name.is_empty() {
                    String::from("RESTING...")
                } else {
                    format!("RESTING FROM: {}", session.task_name.as_str())
                }
            }
        }
    } else {
        String::from("IDLE...")
    };

    let display_msg = if message.len() > 45 {
        format!("{}...", &message[..42])
    } else {
        message
    };

    let bubble_art = speech_bubble::create(&display_msg);

    f.render_widget(
        Paragraph::new(bubble_art)
            .alignment(Alignment::Center)
            .cyan(),
        chunks[0],
    );

    let current_frame = if frame_count.is_multiple_of(2) {
        DOG_FRAME_1
    } else {
        DOG_FRAME_2
    };
    f.render_widget(
        Paragraph::new(current_frame)
            .alignment(Alignment::Center)
            .yellow(),
        chunks[1],
    );
}
