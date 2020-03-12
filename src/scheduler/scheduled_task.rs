use crate::types::id::TaskId;

pub struct ScheduledTask {
    id : TaskId,
    action : Box<dyn FnMut()>
}

impl ScheduledTask {
    pub fn new <Action : FnMut() + 'static> (id : TaskId, action : Action) -> ScheduledTask {
        ScheduledTask {
            id,
            action : Box::new(action)
        }
    }

    fn get_id(&self) -> TaskId {
        self.id
    }

    pub fn execute(&mut self) {
        (self.action)()
    }
}