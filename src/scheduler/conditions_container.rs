use crate::tasks_conditions::task_condition::TaskCondition;
use crate::types::task_id::TaskId;
use crate::scheduler::conditions::Conditions;
use std::time::Duration;

pub struct ConditionContainer {
    containers : Vec<Conditions>
}

impl ConditionContainer {
    pub fn update_and_get_triggered(&mut self, dt : Duration) -> Vec<TaskId> {
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
}