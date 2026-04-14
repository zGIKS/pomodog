#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    #[default]
    Work,
    Break,
}

pub const MAX_TASK_NAME_LEN: usize = 25;

#[derive(Default, Clone, PartialEq, Eq)]
pub struct TaskName(String);

impl TaskName {
    pub fn add_char(&mut self, c: char) {
        if self.0.len() < MAX_TASK_NAME_LEN && is_valid_input_char(c) {
            self.0.push(c);
        }
    }

    pub fn remove_char(&mut self) {
        self.0.pop();
    }

    pub fn remove_word(&mut self) {
        if let Some(last_space_idx) = self.0.trim_end().rfind(' ') {
            self.0.truncate(last_space_idx + 1);
        } else {
            self.0.clear();
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

fn is_valid_input_char(c: char) -> bool {
    !c.is_control() && c != '\u{200B}' && c != '\u{200C}' && c != '\u{200D}' && c != '\u{FEFF}'
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum InputError {
    #[default]
    None,
    Empty,
}

impl InputError {
    pub fn message(&self) -> &'static str {
        match self {
            InputError::None => "",
            InputError::Empty => "Task name cannot be empty!",
        }
    }

    pub fn has_error(&self) -> bool {
        matches!(self, InputError::Empty)
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Timer {
    remaining_secs: u32,
    total_secs: u32,
}

impl Timer {
    pub fn reset(&mut self, total_secs: u32) {
        self.total_secs = total_secs;
        self.remaining_secs = total_secs;
    }

    pub fn tick(&mut self) -> bool {
        if self.remaining_secs > 0 {
            self.remaining_secs -= 1;
            false
        } else {
            true
        }
    }

    pub fn progress_ratio(&self) -> f32 {
        if self.total_secs == 0 {
            return 0.0;
        }
        (self.total_secs - self.remaining_secs) as f32 / self.total_secs as f32
    }

    pub fn formatted_time(&self) -> String {
        let min = self.remaining_secs / 60;
        let sec = self.remaining_secs % 60;
        format!("{:02}:{:02}", min, sec)
    }
}
