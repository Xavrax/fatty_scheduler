use crate::tasks_conditions::task_condition::TaskCondition;
use std::time::Duration;

pub struct PredicateCondition <'condition_lifetime> {
    predicate : Box<dyn Fn() -> bool + 'condition_lifetime>,
    finished : bool,
    should_trigger : bool
}

impl <'condition_lifetime> PredicateCondition <'condition_lifetime> {
    pub fn new(predicate : impl Fn() -> bool + 'condition_lifetime) -> PredicateCondition <'condition_lifetime> {
        PredicateCondition {
            predicate : Box::new(predicate),
            finished : false,
            should_trigger: false
        }
    }
}

impl <'condition_lifetime> TaskCondition for PredicateCondition <'condition_lifetime> {
    fn is_finished(&self) -> bool {
        self.finished
    }

    fn should_trigger(&self) -> bool {
        self.should_trigger
    }

    fn update(&mut self, _dt: &Duration) {
        if (self.predicate)() {
            self.should_trigger = true;
        }
    }

    fn finish(&mut self) {
        self.finished = true;
    }
}