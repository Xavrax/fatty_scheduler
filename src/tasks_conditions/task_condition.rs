use crate::types::id::TaskId;

pub trait TaskCondition {
    fn is_finished(&self) -> bool;
    fn should_trigger(&self) -> bool;
    fn update(&mut self, dt : &std::time::Duration);
    fn finish(&mut self);
}