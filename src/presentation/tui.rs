use super::components::{dog, menu, progress_bar, task_input};
use crate::domain::{App, AppState};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

pub fn render(f: &mut Frame, app: &App, frame_count: usize) {
    let area = f.area();

    match app.state() {
        AppState::Menu => {
            menu::render(f, app, area);
        }
        AppState::TaskInput => {
            task_input::render(f, app, area, frame_count);
        }
        AppState::Running | AppState::Paused => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Fill(1),
                    Constraint::Length(11),
                    Constraint::Length(5),
                    Constraint::Fill(1),
                ])
                .split(area);

            dog::render(f, app, chunks[1], frame_count);
            progress_bar::render(f, app, chunks[2]);
        }
    }
}
