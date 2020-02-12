use crate::scheduler::conditions_container::ConditionContainer;
use crate::scheduler::scheduled_task::ScheduledTask;

struct Scheduler<Action : Fn()>{
    conditions : ConditionContainer,
    tasks : Vec<ScheduledTask<Action>>
}

impl <Action : Fn()> Scheduler<Action> {

}