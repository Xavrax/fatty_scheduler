use crate::types::id::TaskId;

pub struct ScheduledTask<Predicate : FnMut()> {
    id : TaskId,
    action : Predicate
}

impl <Predicate : FnMut()> ScheduledTask<Predicate> {
    pub fn new (id : TaskId, action : Predicate) -> ScheduledTask<Predicate> {
        ScheduledTask {
            id,
            action
        }
    }

    fn get_id(&self) -> TaskId {
        self.id
    }

    pub fn execute(&mut self) {
        (self.action)()
    }
}