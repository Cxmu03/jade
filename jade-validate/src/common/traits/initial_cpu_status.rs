use crate::common::types::{CpuSnapshot, ExecutionError};

pub trait Init {
    fn new() -> Self;
}

pub trait HasInitialCpuStatus {
    fn reset(&mut self) -> Result<(CpuSnapshot, u16), ExecutionError>;
}

pub trait InitializeWithCpuStatus {
    fn init_with_cpu_status(&mut self, status: &CpuSnapshot, new_pc: u16);
}
