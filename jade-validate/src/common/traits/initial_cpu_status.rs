use crate::common::types::{CpuSnapshot, ExecutionError};

pub trait HasInitialCpuStatus {
    fn new() -> Self;
    fn reset(&mut self) -> Result<(CpuSnapshot, u16), ExecutionError>;
}

pub trait InitializeWithCpuStatus {
    fn new() -> Self;
    fn init_with_cpu_status(&mut self, status: &CpuSnapshot, new_pc: u16);
}
