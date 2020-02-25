use crate::tasks_conditions::task_condition::TaskCondition;
use crate::tasks_conditions::delay_condition::DelayCondition;
use crate::tasks_conditions::repetitive_condition::RepetitiveCondition;
use crate::tasks_conditions::predicate_condition::PredicateCondition;
use std::time::Duration;

pub struct SchedulingBuilder {
    conditions : Vec<Box<dyn TaskCondition>>
}

impl SchedulingBuilder {
    pub fn new() -> SchedulingBuilder {
        SchedulingBuilder {
            conditions :  Vec::<Box<dyn TaskCondition>>::new()
        }
    }

    pub fn after(&mut self, delay : Duration) -> &mut SchedulingBuilder {
        self.conditions.push(Box::new(DelayCondition::new(delay)));
        self
    }

    pub fn every(&mut self, interval : Duration) -> &mut SchedulingBuilder {
        self.conditions.push(Box::new(RepetitiveCondition::new(interval)));
        self
    }

    pub fn when(&mut self, predicate : impl Fn() -> bool) -> &mut SchedulingBuilder {
       self.conditions.push(Box::new(PredicateCondition::new(predicate)));
       self
    }
}