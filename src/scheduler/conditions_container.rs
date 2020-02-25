use crate::tasks_conditions::task_condition::TaskCondition;
use crate::types::id::TaskId;
use crate::scheduler::conditions::Conditions;
use std::time::Duration;

pub struct ConditionContainer {
    containers : Vec<Conditions>
}

impl ConditionContainer {
    pub fn new() -> ConditionContainer {
        ConditionContainer{
            containers : Vec::<Conditions>::new()
        }
    }

//    pub fn add(&mut self, id : TaskId) {
//        self.containers.push(Conditions::new(id))
//    }

    pub fn update_and_get_triggered(&mut self, dt : &Duration) -> Vec<TaskId> {
        let mut triggered = Vec::new();

        self.containers
            .iter_mut()
            .for_each(|c|{
                if c.are_finished() {
                    c.update(dt);

                    if c.should_trigger() {
                        triggered.push(c.get_id())
                    }
                }
            }
        );

        triggered
    }

    pub fn trigger(&mut self, id : TaskId) {
        match self.containers
            .iter_mut()
            .into_iter()
            .find(|c| c.get_id() == id) {
            Some(mut c) => c.trigger(),
            _ => println!("Task with id {} does not exists!", id)
        };
    }
}