use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use super::components::{dog, menu, progress_bar, task_input};
use crate::domain::{App, AppState};

pub fn render(f: &mut Frame, app: &App) {
    let area = f.area();

    match app.state() {
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
                    Constraint::Length(12),
                    Constraint::Length(6),
                    Constraint::Fill(1),
                ])
                .split(area);

            dog::render(f, app, chunks[1]);
            progress_bar::render(f, app, chunks[2]);
        }
    }
}
