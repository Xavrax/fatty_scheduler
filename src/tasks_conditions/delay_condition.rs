use std::time::Duration;
use crate::tasks_conditions::task_condition::TaskCondition;

struct DelayCondition {
    timeout : Duration,
    finished : bool,
    should_trigger : bool,
}

impl TaskCondition for DelayCondition {
    fn is_finished(&self) -> bool {
        self.finished
    }

    fn should_trigger(&self) -> bool {
        self.should_trigger
    }

    fn update(&mut self, dt: &Duration) {
        if self.should_trigger {
            return;
        }

        if self.timeout <= *dt {
            self.should_trigger = true
        } else {
            self.timeout -= *dt;
        }
    }

    fn finish(&mut self) {
        self.finished = true;
    }
}