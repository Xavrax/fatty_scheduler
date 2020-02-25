use crate::scheduler::conditions_container::ConditionContainer;
use crate::scheduler::scheduled_task::ScheduledTask;

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

//    fn schedule(&mut self) -> Sche{
//
//    }
}