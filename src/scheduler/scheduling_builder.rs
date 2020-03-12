use crate::tasks_conditions::task_condition::TaskCondition;
use crate::tasks_conditions::delay_condition::DelayCondition;
use crate::tasks_conditions::repetitive_condition::RepetitiveCondition;
use crate::tasks_conditions::predicate_condition::PredicateCondition;
use std::time::Duration;
use crate::scheduler::scheduler::Scheduler;
use crate::scheduler::scheduled_task::ScheduledTask;
use std::collections::VecDeque;
use crate::scheduler::repetitive_scheduling_builder::RepetitiveSchedulingBuilder;

pub struct SchedulingBuilder<'scheduler_lifetime> {
    scheduler : &'scheduler_lifetime mut Scheduler,
    conditions : VecDeque<Box<dyn TaskCondition>>
}

impl <'scheduler_lifetime> SchedulingBuilder<'scheduler_lifetime> {
    pub fn new(scheduler : &'scheduler_lifetime mut Scheduler) -> SchedulingBuilder<'scheduler_lifetime> {
        SchedulingBuilder {
            scheduler,
            conditions :  VecDeque::<Box<dyn TaskCondition>>::new()
        }
    }

    pub fn after(&mut self, delay : Duration) -> &'scheduler_lifetime mut SchedulingBuilder {
        self.conditions.push_back(Box::new(DelayCondition::new(delay)));
        self
    }

    pub fn every(&mut self, interval : Duration) -> RepetitiveSchedulingBuilder {
        RepetitiveSchedulingBuilder::new(self.scheduler, interval)
    }

    pub fn when(&mut self, predicate : impl Fn() -> bool + 'static) -> &'scheduler_lifetime mut SchedulingBuilder {
        self.conditions.push_back(Box::new(PredicateCondition::new(predicate)));
        self
    }

    pub fn execute <Action : FnMut() + 'static>(&mut self, action : Action) {
        self.scheduler.add_task(std::mem::replace(&mut self.conditions, VecDeque::new()), ScheduledTask::new(0, action))
    }
}