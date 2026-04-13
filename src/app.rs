#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum AppState {
    #[default]
    Menu,
    TaskInput,
    Running,
    Paused,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    #[default]
    Work,
    Break,
}

pub struct SessionConfig {
    pub label: String,
    pub work_duration_min: u32,
    pub break_duration_min: u32,
}

pub const MAX_TASK_NAME_LEN: usize = 25;

pub struct App {
    pub frame_count: usize,
    pub should_quit: bool,
    pub state: AppState,
    pub phase: Phase,
    pub configs: Vec<SessionConfig>,
    pub selected_index: usize,
    pub remaining_secs: u32,
    pub total_secs: u32,
    pub task_name: String,
}

impl App {
    pub fn new() -> Self {
        let configs = vec![
            SessionConfig { label: String::from("Classic (25/5)"), work_duration_min: 25, break_duration_min: 5 },
            SessionConfig { label: String::from("Focus (50/10)"), work_duration_min: 50, break_duration_min: 10 },
            SessionConfig { label: String::from("Quick (15/5)"), work_duration_min: 15, break_duration_min: 5 },
        ];

        Self {
            frame_count: 0,
            should_quit: false,
            state: AppState::Menu,
            phase: Phase::Work,
            configs,
            selected_index: 0,
            remaining_secs: 0,
            total_secs: 0,
            task_name: String::new(),
        }
    }

    pub fn next_option(&mut self) {
        if self.configs.is_empty() { return; }
        self.selected_index = (self.selected_index + 1) % self.configs.len();
    }

    pub fn prev_option(&mut self) {
        if self.configs.is_empty() { return; }
        if self.selected_index == 0 {
            self.selected_index = self.configs.len() - 1;
        } else {
            self.selected_index -= 1;
        }
    }

    pub fn enter_task_input(&mut self) {
        self.state = AppState::TaskInput;
    }

    pub fn start_session(&mut self) {
        let config = &self.configs[self.selected_index];
        self.phase = Phase::Work;
        self.total_secs = config.work_duration_min * 60;
        self.remaining_secs = self.total_secs;
        self.state = AppState::Running;
    }

    pub fn add_char(&mut self, c: char) {
        if self.task_name.len() < MAX_TASK_NAME_LEN {
            self.task_name.push(c);
        }
    }

    pub fn remove_char(&mut self) {
        self.task_name.pop();
    }

    pub fn tick(&mut self) {
        if self.state == AppState::Running {
            if self.remaining_secs > 0 {
                self.remaining_secs -= 1;
            } else {
                self.transition_phase();
            }
        }
    }

    fn transition_phase(&mut self) {
        let config = &self.configs[self.selected_index];
        match self.phase {
            Phase::Work => {
                self.phase = Phase::Break;
                self.total_secs = config.break_duration_min * 60;
            }
            Phase::Break => {
                self.phase = Phase::Work;
                self.total_secs = config.work_duration_min * 60;
            }
        }
        self.remaining_secs = self.total_secs;
    }

    pub fn progress_ratio(&self) -> f32 {
        if self.total_secs == 0 { return 0.0; }
        (self.total_secs - self.remaining_secs) as f32 / self.total_secs as f32
    }

    pub fn formatted_time(&self) -> String {
        let min = self.remaining_secs / 60;
        let sec = self.remaining_secs % 60;
        format!("{:02}:{:02}", min, sec)
    }

    pub fn toggle_pause(&mut self) {
        match self.state {
            AppState::Running => self.state = AppState::Paused,
            AppState::Paused => self.state = AppState::Running,
            _ => {}
        }
    }

    pub fn update_frame(&mut self) {
        self.frame_count += 1;
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
