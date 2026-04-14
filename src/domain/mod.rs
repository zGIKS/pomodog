pub mod entities;
pub mod value_objects;

pub use entities::*;
pub use value_objects::*;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum AppState {
    #[default]
    Menu,
    TaskInput,
    Running,
    Paused,
}

pub struct App {
    frame_count: usize,
    should_quit: bool,
    state: AppState,
    phase: Phase,
    configs: Vec<SessionConfig>,
    selected_index: usize,
    timer: Timer,
    task_name: TaskName,
    input_error: InputError,
    error_timer: u32,
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
            frame_count: 0,
            should_quit: false,
            state: AppState::Menu,
            phase: Phase::Work,
            configs,
            selected_index: 0,
            timer: Timer::default(),
            task_name: TaskName::default(),
            input_error: InputError::default(),
            error_timer: 0,
        }
    }

    pub fn next_option(&mut self) {
        if self.configs.is_empty() {
            return;
        }
        self.selected_index = (self.selected_index + 1) % self.configs.len();
    }

    pub fn prev_option(&mut self) {
        if self.configs.is_empty() {
            return;
        }
        if self.selected_index == 0 {
            self.selected_index = self.configs.len() - 1;
        } else {
            self.selected_index -= 1;
        }
    }

    pub fn enter_task_input(&mut self) {
        self.state = AppState::TaskInput;
        self.task_name.clear();
        self.input_error = InputError::default();
        self.error_timer = 0;
    }

    pub fn enter_menu(&mut self) {
        self.state = AppState::Menu;
        self.task_name.clear();
        self.input_error = InputError::default();
        self.error_timer = 0;
    }

    pub fn start_session(&mut self) {
        if self.configs.is_empty() {
            return;
        }
        if self.task_name.is_empty() {
            self.set_input_error(InputError::Empty);
            return;
        }
        let config = &self.configs[self.selected_index];
        self.phase = Phase::Work;
        self.timer.reset(config.work_duration_min * 60);
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
        if self.state == AppState::Running && self.timer.tick() {
            self.transition_phase();
        }
    }

    pub fn update_frame(&mut self) {
        self.frame_count += 1;
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

    pub fn phase(&self) -> Phase {
        self.phase
    }

    pub fn frame_count(&self) -> usize {
        self.frame_count
    }

    pub fn task_name(&self) -> &TaskName {
        &self.task_name
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn configs(&self) -> &[SessionConfig] {
        &self.configs
    }

    pub fn timer(&self) -> &Timer {
        &self.timer
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn add_char_to_task(&mut self, c: char) {
        self.task_name.add_char(c);
    }

    pub fn remove_char_from_task(&mut self) {
        self.task_name.remove_char();
    }

    pub fn remove_word_from_task(&mut self) {
        self.task_name.remove_word();
    }

    fn transition_phase(&mut self) {
        let config = &self.configs[self.selected_index];
        match self.phase {
            Phase::Work => {
                self.phase = Phase::Break;
                self.timer.reset(config.break_duration_min * 60);
            }
            Phase::Break => {
                self.phase = Phase::Work;
                self.timer.reset(config.work_duration_min * 60);
            }
        }
    }

    pub fn toggle_pause(&mut self) {
        match self.state {
            AppState::Running => self.state = AppState::Paused,
            AppState::Paused => self.state = AppState::Running,
            _ => {}
        }
    }
}
