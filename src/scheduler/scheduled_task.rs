pub struct ScheduledTask {
    action : Box<dyn FnMut()>
}

impl ScheduledTask {
    pub fn new <Action : FnMut() + 'static> (action : Action) -> ScheduledTask {
        ScheduledTask {
            action : Box::new(action)
        }
    }

    pub fn execute(&mut self) {
        (self.action)()
    }
}