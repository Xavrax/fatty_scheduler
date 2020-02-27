use crate::tasks_conditions::task_condition::TaskCondition;
use crate::tasks_conditions::delay_condition::DelayCondition;
use crate::tasks_conditions::repetitive_condition::RepetitiveCondition;
use crate::tasks_conditions::predicate_condition::PredicateCondition;
use std::time::Duration;
use crate::scheduler::scheduler::Scheduler;
use crate::scheduler::scheduled_task::ScheduledTask;

pub struct SchedulingBuilder<'scheduler_lifetime, Action : FnMut()> {
    scheduler : &'scheduler_lifetime mut Scheduler<Action>,
    conditions : Vec<Box<dyn TaskCondition>>
}

impl <'scheduler_lifetime, Action : FnMut()> SchedulingBuilder<'scheduler_lifetime, Action > {
    pub fn new(scheduler : &'scheduler_lifetime mut Scheduler<Action>) -> SchedulingBuilder <'scheduler_lifetime, Action> {
        SchedulingBuilder {
            scheduler,
            conditions :  Vec::<Box<dyn TaskCondition>>::new()
        }
    }

    pub fn after(&mut self, delay : Duration) -> &'scheduler_lifetime mut SchedulingBuilder<Action> {
        self.conditions.push(Box::new(DelayCondition::new(delay)));
        self
    }

    pub fn every(&mut self, interval : Duration) -> &'scheduler_lifetime mut SchedulingBuilder<Action> {
        self.conditions.push(Box::new(RepetitiveCondition::new(interval)));
        self
    }

    pub fn when(&mut self, predicate : impl Fn() -> bool + 'static) -> &'scheduler_lifetime mut SchedulingBuilder<Action> {
       self.conditions.push(Box::new(PredicateCondition::new(predicate)));
       self
    }

    pub fn execute(&mut self, action : Action) {
        self.scheduler.add_task(std::mem::replace(&mut self.conditions, Vec::new()), ScheduledTask::new(0, action))
    }
}