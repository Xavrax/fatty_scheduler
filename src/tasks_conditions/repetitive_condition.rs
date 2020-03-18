use crate::tasks_conditions::task_condition::TaskCondition;
use std::time::Duration;

pub struct RepetitiveCondition<'condition_lifetime> {
    interval : Duration,
    timeout : Duration,
    should_trigger : bool,
    stop_condition : Box<dyn Fn() -> bool + 'condition_lifetime>
}

impl<'condition_lifetime> RepetitiveCondition<'condition_lifetime> {
    pub fn new (interval : Duration, stop_condition : Box<dyn Fn() -> bool + 'condition_lifetime>) -> RepetitiveCondition<'condition_lifetime> {
        let timeout = interval.clone();
        RepetitiveCondition {
            interval,
            timeout,
            should_trigger : false,
            stop_condition
        }
    }
}

impl<'condition_lifetime> TaskCondition for RepetitiveCondition<'condition_lifetime> {
    fn is_finished(&self) -> bool {
        !(*self.stop_condition)()
    }

    fn should_trigger(&self) -> bool {
        self.should_trigger && !self.is_finished()
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