use crate::domain::value_objects::{Phase, TaskName, Timer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionConfig {
    pub label: String,
    pub work_duration_min: u32,
    pub break_duration_min: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub task_name: TaskName,
    pub phase: Phase,
    pub timer: Timer,
    pub config: SessionConfig,
}

impl Session {
    pub fn new(task_name: TaskName, config: SessionConfig) -> Self {
        let mut timer = Timer::default();
        timer.reset(config.work_duration_min * 60);
        Self {
            task_name,
            phase: Phase::Work,
            timer,
            config,
        }
    }

    pub fn tick(&mut self) -> bool {
        if self.timer.tick() {
            self.transition_phase();
            true
        } else {
            false
        }
    }

    fn transition_phase(&mut self) {
        match self.phase {
            Phase::Work => {
                self.phase = Phase::Break;
                self.timer.reset(self.config.break_duration_min * 60);
            }
            Phase::Break => {
                self.phase = Phase::Work;
                self.timer.reset(self.config.work_duration_min * 60);
            }
        }
    }
}
