pub mod entities;
pub mod repository;
pub mod value_objects;

pub use entities::*;
pub use value_objects::*;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum AppState {
    #[default]
    Menu,
    TaskInput,
    Running,
    Paused,
}

#[derive(Serialize, Deserialize)]
pub struct App {
    state: AppState,
    #[serde(skip)]
    recovered_session: Option<Session>,
    #[serde(skip)]
    has_saved_session: bool,
    configs: Vec<SessionConfig>,
    selected_index: usize,
    active_session: Option<Session>,
    task_name_input: TaskName,
    #[serde(skip)]
    input_error: InputError,
    #[serde(skip)]
    error_timer: u32,
    #[serde(skip)]
    should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let configs = vec![
            SessionConfig {
                label: String::from("Classic (25/5)"),
                work_duration_min: 25,
                break_duration_min: 5,
            },
            SessionConfig {
                label: String::from("Focus (50/10)"),
                work_duration_min: 50,
                break_duration_min: 10,
            },
            SessionConfig {
                label: String::from("Quick (15/5)"),
                work_duration_min: 15,
                break_duration_min: 5,
            },
        ];

        Self {
            state: AppState::Menu,
            recovered_session: None,
            has_saved_session: false,
            configs,
            selected_index: 0,
            active_session: None,
            task_name_input: TaskName::default(),
            input_error: InputError::default(),
            error_timer: 0,
            should_quit: false,
        }
    }

    pub fn set_resume_session(&mut self, session: Session) {
        self.recovered_session = Some(session);
        self.active_session = None;
        self.has_saved_session = true;
        self.state = AppState::Menu;
        self.selected_index = 0;
    }

    pub fn resume_session(&mut self) {
        if let Some(session) = self.recovered_session.take() {
            self.active_session = Some(session);
            self.state = AppState::Running;
            self.has_saved_session = false;
        }
    }

    pub fn discard_saved_session(&mut self) {
        self.has_saved_session = false;
        self.recovered_session = None;
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    fn total_options(&self) -> usize {
        if self.has_saved_session {
            self.configs.len() + 1
        } else {
            self.configs.len()
        }
    }

    pub fn next_option(&mut self) {
        let total = self.total_options();
        if total == 0 {
            return;
        }
        self.selected_index = (self.selected_index + 1) % total;
    }

    pub fn prev_option(&mut self) {
        let total = self.total_options();
        if total == 0 {
            return;
        }
        if self.selected_index == 0 {
            self.selected_index = total - 1;
        } else {
            self.selected_index -= 1;
        }
    }

    pub fn has_saved_session(&self) -> bool {
        self.has_saved_session
    }

    pub fn select_current_option(&mut self) {
        if self.has_saved_session {
            if self.selected_index == 0 {
                self.resume_session();
            } else {
                self.discard_saved_session();
                self.enter_task_input();
            }
        } else {
            self.enter_task_input();
        }
    }

    pub fn enter_task_input(&mut self) {
        self.state = AppState::TaskInput;
        self.reset_input_state();
    }

    pub fn enter_menu(&mut self) {
        self.state = AppState::Menu;
        self.reset_input_state();
    }

    fn reset_input_state(&mut self) {
        self.task_name_input.clear();
        self.input_error = InputError::default();
        self.error_timer = 0;
    }

    pub fn start_session(&mut self) {
        if self.configs.is_empty() {
            return;
        }
        if self.task_name_input.is_empty() {
            self.set_input_error(InputError::Empty);
            return;
        }
        let config = self.configs[self.selected_index].clone();
        self.active_session = Some(Session::new(self.task_name_input.clone(), config));
        self.state = AppState::Running;
        self.input_error = InputError::default();
        self.error_timer = 0;
    }

    pub fn set_input_error(&mut self, error: InputError) {
        self.input_error = error;
        self.error_timer = 60;
    }

    pub fn clear_input_error(&mut self) {
        self.input_error = InputError::default();
        self.error_timer = 0;
    }

    pub fn get_input_error(&self) -> Option<&'static str> {
        if self.input_error.has_error() {
            Some(self.input_error.message())
        } else {
            None
        }
    }

    pub fn tick(&mut self) {
        if let (AppState::Running, Some(session)) = (self.state, &mut self.active_session) {
            session.tick();
        }
    }

    pub fn update_error_timer(&mut self) {
        if self.error_timer > 0 {
            self.error_timer -= 1;
            if self.error_timer == 0 {
                self.input_error = InputError::default();
            }
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn state(&self) -> AppState {
        self.state
    }

    pub fn task_name(&self) -> &TaskName {
        if let Some(session) = &self.active_session {
            &session.task_name
        } else {
            &self.task_name_input
        }
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn configs(&self) -> &[SessionConfig] {
        &self.configs
    }

    pub fn session(&self) -> Option<&Session> {
        self.active_session.as_ref()
    }

    pub fn take_session(&mut self) -> Option<Session> {
        self.active_session.take()
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn add_char_to_task(&mut self, c: char) {
        self.task_name_input.add_char(c);
    }

    pub fn remove_char_from_task(&mut self) {
        self.task_name_input.remove_char();
    }

    pub fn remove_word_from_task(&mut self) {
        self.task_name_input.remove_word();
    }

    pub fn toggle_pause(&mut self) {
        match self.state {
            AppState::Running => self.state = AppState::Paused,
            AppState::Paused => self.state = AppState::Running,
            _ => {}
        }
    }
}
