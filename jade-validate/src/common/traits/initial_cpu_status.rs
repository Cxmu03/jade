use crate::common::types::CpuSnapshot;

pub trait HasInitialCpuStatus {
    fn new() -> Self;
    fn get_initial_cpu_status(&self) -> CpuSnapshot;
}

pub trait InitializeWithCpuStatus {
    fn new() -> Self;
    fn init_with_cpu_status(&mut self, status: &CpuSnapshot);
}
