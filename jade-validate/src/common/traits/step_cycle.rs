use crate::common::types::ExecutionError;

pub trait StepCycle {
    fn step_cycle(&mut self) -> Result<(), ExecutionError>;
}
