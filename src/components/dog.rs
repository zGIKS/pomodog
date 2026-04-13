use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Stylize,
    widgets::Paragraph,
    Frame,
};

use crate::app::{App, Phase};
use crate::components::speech_bubble;

// Refined Dog ASCII art frames
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

pub fn render(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Speech Bubble
            Constraint::Length(6), // Dog
        ])
        .split(area);

    // 1. Render Speech Bubble (Focus on Task message, no time)
    let message = match app.phase {
        Phase::Work => {
            if app.task_name.is_empty() {
                String::from("WORKING...")
            } else {
                format!("FOCUSING ON: {}", app.task_name)
            }
        }
        Phase::Break => {
            if app.task_name.is_empty() {
                String::from("RESTING...")
            } else {
                format!("RESTING FROM: {}", app.task_name)
            }
        }
    };

    // Truncate message if it's too long for the bubble
    let display_msg = if message.len() > 35 {
        format!("{}...", &message[..32])
    } else {
        message
    };

    let bubble_art = speech_bubble::create(&display_msg);
    
    f.render_widget(Paragraph::new(bubble_art).alignment(Alignment::Center).cyan(), chunks[0]);

    // 2. Render Dog
    let current_frame = if app.frame_count % 2 == 0 {
        DOG_FRAME_1
    } else {
        DOG_FRAME_2
    };
    f.render_widget(Paragraph::new(current_frame).alignment(Alignment::Center).yellow(), chunks[1]);
}
