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

static MSG_IDLE: &str = "IDLE...";
static MSG_PAUSED: &str = "PAUSED...";

fn status_prefix(app: &App) -> &'static str {
    match app.state() {
        AppState::Paused => MSG_PAUSED,
        AppState::Running | AppState::TaskInput => {
            if let Some(session) = app.session() {
                match session.phase {
                    Phase::Work if session.task_name.is_empty() => "WORKING...",
                    Phase::Break if session.task_name.is_empty() => "RESTING...",
                    Phase::Work => "FOCUSING ON: ",
                    Phase::Break => "RESTING FROM: ",
                }
            } else {
                MSG_IDLE
            }
        }
        AppState::Menu => MSG_IDLE,
    }
}

pub fn render(f: &mut Frame, app: &App, area: ratatui::layout::Rect, frame_count: usize) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Length(6)])
        .split(area);

    let prefix = status_prefix(app);
    let task_name = app.session().map(|s| s.task_name.as_str()).unwrap_or("");

    let display_msg = if task_name.is_empty() {
        prefix.to_string()
    } else {
        let combined = format!("{}{}", prefix, task_name);
        if combined.len() > 45 {
            format!("{}...", &combined[..42])
        } else {
            combined
        }
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
