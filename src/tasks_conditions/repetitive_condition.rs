use std::time::Duration;
use crate::tasks_conditions::task_condition::TaskCondition;

struct RepetitiveCondition {
    interval : Duration,
    timeout : Duration,
    should_trigger : bool,
}

impl TaskCondition for RepetitiveCondition {
    fn is_finished(&self) -> bool {
        false
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
        self.should_trigger = false;
        self.timeout = self.interval.clone();
    }
}