use crate::tasks_conditions::task_condition::TaskCondition;
use std::time::Duration;

pub struct PredicateCondition<Predicate : Fn() -> bool> {
    predicate : Predicate,
    finished : bool,
    should_trigger : bool
}

impl<Predicate : Fn() -> bool> TaskCondition for PredicateCondition<Predicate> {
    fn is_finished(&self) -> bool {
        self.finished
    }

    fn should_trigger(&self) -> bool {
        self.should_trigger
    }

    fn update(&mut self, dt: Duration) {
        if self.predicate() {
            self.should_trigger = true;
        }
    }

    fn finish(&mut self) {
        self.finished = true;
    }
}