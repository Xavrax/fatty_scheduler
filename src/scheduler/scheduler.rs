use crate::scheduler::conditions_container::ConditionContainer;
use crate::scheduler::scheduled_task::ScheduledTask;
use crate::scheduler::scheduling_builder::SchedulingBuilder;
use std::time::Duration;
use crate::tasks_conditions::task_condition::TaskCondition;
use crate::scheduler::conditions::Conditions;
use std::collections::VecDeque;
use std::borrow::{Borrow, BorrowMut};

pub struct Scheduler {
    // conditions : ConditionContainer,
    condition_chains : Vec<VecDeque<Box<dyn TaskCondition>>>,
    conditions : Vec<Box<dyn TaskCondition>>,
    tasks : Vec<ScheduledTask>
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            condition_chains : Vec::new(),
            conditions : Vec::new(),
            tasks : Vec::<ScheduledTask>::new()
        }
    }

   pub fn update(&mut self, dt : &Duration) {
       let mut index = 0usize;
       let mut to_chain_pop = Vec::new();
       let mut to_remove = Vec::new();

       for mut condition in &mut self.conditions {
           condition.update(&dt);
           if condition.should_trigger() {
               condition.finish();

               if self.condition_chains[index].is_empty() {
                   self.tasks[index].execute();

                   if condition.is_finished() {
                        to_remove.push(index);
                   }
               } else {
                   to_chain_pop.push(index);
               }
           }

           index += 1;
       }

       to_chain_pop.iter()
           .for_each(|i| self.conditions[*i] = self.condition_chains[*i].pop_front().unwrap());

       to_remove.iter()
           .for_each(|i| self.remove_task(*i))
   }

   pub fn schedule(&mut self) -> SchedulingBuilder {
       SchedulingBuilder::new(self)
   }

   pub fn add_task(&mut self, mut conditions: VecDeque<Box<dyn TaskCondition>>, task : ScheduledTask) {
       self.tasks.push(task);
       self.conditions.push(conditions.pop_front().unwrap());
       self.condition_chains.push(conditions);
   }

    fn remove_task(&mut self, index : usize) {
        self.tasks.remove(index);
        self.conditions.remove(index);
        self.condition_chains.remove(index);
    }
}