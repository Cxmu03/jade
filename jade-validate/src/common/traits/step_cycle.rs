use crate::common::types::{CpuSnapshot, ExecutionError};

pub trait StepCycle {
    fn step_cycle(&mut self) -> Result<CpuSnapshot, ExecutionError>;
}
