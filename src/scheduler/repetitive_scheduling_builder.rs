use crate::scheduler::scheduler::Scheduler;
use crate::tasks_conditions::task_condition::TaskCondition;
use crate::tasks_conditions::repetitive_condition::RepetitiveCondition;
use std::time::Duration;
use std::collections::VecDeque;
use crate::scheduler::scheduled_task::ScheduledTask;

pub struct RepetitiveSchedulingBuilder<'scheduler_lifetime> {
    scheduler : &'scheduler_lifetime mut Scheduler,
    conditions : VecDeque<Box<dyn TaskCondition>>,
    interval : Duration,
    stop_condition : Box<dyn Fn() -> bool>
}

impl <'scheduler_lifetime> RepetitiveSchedulingBuilder<'scheduler_lifetime> {
    pub fn new(scheduler: &'scheduler_lifetime mut Scheduler, interval : Duration) -> RepetitiveSchedulingBuilder<'scheduler_lifetime> {
        RepetitiveSchedulingBuilder {
            scheduler,
            conditions: VecDeque::<Box<dyn TaskCondition>>::new(),
            interval,
            stop_condition : Box::new(|| true)
        }
    }

    pub fn as_long_as(&mut self, predicate : impl Fn() -> bool + 'static) -> &'scheduler_lifetime mut RepetitiveSchedulingBuilder {
        self.stop_condition = Box::new(predicate);
        self
    }

    pub fn execute <Action : FnMut() + 'static>(&mut self, action : Action) -> Result<usize, &str>{
        let stop = std::mem::replace(&mut self.stop_condition, Box::new(|| true));
        self.conditions.push_back(Box::new(RepetitiveCondition::new(self.interval, stop)));
        self.scheduler.add_task(std::mem::replace(&mut self.conditions, VecDeque::new()), ScheduledTask::new(action))
    }
}
