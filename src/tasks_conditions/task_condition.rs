use crate::types::task_id::TaskId;

trait TaskCondition {
    fn get_task_id() -> TaskId;
    fn is_finished() -> bool;
    fn should_trigger() -> bool;
    fn update(dt : std::time::Duration);
}