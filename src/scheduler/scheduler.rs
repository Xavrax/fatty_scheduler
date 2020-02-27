use crate::scheduler::conditions_container::ConditionContainer;
use crate::scheduler::scheduled_task::ScheduledTask;
use crate::scheduler::scheduling_builder::SchedulingBuilder;
use std::time::Duration;
use crate::tasks_conditions::task_condition::TaskCondition;
use crate::scheduler::conditions::Conditions;

pub struct Scheduler<Action : FnMut()>{
    conditions : ConditionContainer,
    tasks : Vec<ScheduledTask<Action>>
}

impl <Action : FnMut()> Scheduler<Action> {
    pub fn new() -> Scheduler<Action> {
        Scheduler {
            conditions : ConditionContainer::new(),
            tasks : Vec::<ScheduledTask<Action>>::new()
        }
    }

   pub fn update(&mut self, dt : &Duration) {
       let to_trigger = self.conditions.update_and_get_triggered(dt);
       for id in to_trigger {
           self.tasks[id].execute();
       }
   }

   pub fn schedule(&mut self) -> SchedulingBuilder<Action> {
        SchedulingBuilder::new(self)
   }

   pub fn add_task(&mut self, conditions : Vec<Box<dyn TaskCondition>>, task : ScheduledTask<Action>) {
       self.tasks.push(task);
       self.conditions.push(Conditions::new(0, conditions));
   }

}