use crate::tasks_conditions::task_condition::TaskCondition;
use crate::types::id::TaskId;
use std::time::Duration;
use core::borrow::BorrowMut;
use std::io::Write;
use std::any::{TypeId, Any};

pub struct Conditions {
    id : TaskId,
    conditions : Vec<Box<dyn TaskCondition>>
}

impl Conditions {
    pub fn new(id : TaskId, conditions : Vec<Box<dyn TaskCondition>>) -> Conditions {
        Conditions {
            id,
            conditions
        }
    }

    pub fn add(&mut self, condition : Box<dyn TaskCondition>) {
        self.conditions.push(condition);
    }

    pub fn are_finished(&self) -> bool {
        self.conditions
            .iter()
            .all(|c| c.is_finished())

    }

    pub fn should_trigger (&self) -> bool {
        self.conditions
            .iter()
            .all(|c| c.should_trigger())
    }

    pub fn update (&mut self, dt : &Duration) {
        self.conditions
            .iter_mut()
            .for_each(|c| c.update(dt))
    }

    pub fn trigger(&mut self) {
        self.conditions
            .iter_mut()
            .for_each(|c| c.finish())
    }

    pub fn get_id(&self) -> TaskId {
        self.id
    }
}