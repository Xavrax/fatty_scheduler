use crate::tasks_conditions::task_condition::TaskCondition;
use crate::tasks_conditions::delay_condition::DelayCondition;
use crate::tasks_conditions::repetitive_condition::RepetitiveCondition;
use crate::tasks_conditions::predicate_condition::PredicateCondition;
use std::time::Duration;

pub struct SchedulingBuilder {
    conditions : Vec<Box<TaskCondition>>
}

impl SchedulingBuilder {
    fn new() -> SchedulingBuilder {
        let conditions = Vec::<Box<TaskCondition>>::new();
        SchedulingBuilder {
            conditions
        }
    }

    fn after(&mut self, delay : Duration) -> &mut SchedulingBuilder {
        self.conditions.push(Box::new(DelayCondition::new(delay)));
        self
    }

    fn every(&mut self, interval : Duration) -> &mut SchedulingBuilder {
        self.conditions.push(Box::new(RepetitiveCondition::new(interval)));
        self
    }

    fn when<Predicate : Fn() -> bool>(&mut self, predicate : Predicate) -> &mut SchedulingBuilder {
//        self.conditions.push(Box::new(PredicateCondition::new(predicate)));
        self
    }
}