use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::{App, AppState};
use crate::components::{dog, progress_bar, menu, task_input};

/// Core UI Entrypoint
pub fn render(f: &mut Frame, app: &App) {
    let area = f.area();

    match app.state {
        AppState::Menu => {
            menu::render(f, app, area);
        }
        AppState::TaskInput => {
            task_input::render(f, app, area);
        }
        AppState::Running | AppState::Paused => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Fill(1),
                    Constraint::Length(12), // Dog + Speech Bubble
                    Constraint::Length(6),  // Large Timer + Progress Bar
                    Constraint::Fill(1),
                ])
                .split(area);

            // 1. Render the Dog Component
            dog::render(f, app, chunks[1]);

            // 2. Render the Progress & Timer Component
            progress_bar::render(f, app, chunks[2]);
        }
    }
}
