use crate::types::id::TaskId;

pub struct ScheduledTask<Predicate : Fn()> {
    id : TaskId,
    action : Predicate
}

impl <Predicate : Fn()> ScheduledTask<Predicate> {
    fn get_id(&self) -> TaskId {
        self.id
    }

    fn execute(&self) {
        (self.action)()
    }
}