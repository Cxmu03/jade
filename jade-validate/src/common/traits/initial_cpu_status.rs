use crate::common::types::InitialCpuStatus;

pub trait HasInitialCpuStatus {
    fn get_default_cpu_status(&self) -> InitialCpuStatus;
}

pub trait InitializeWithCpuStatus {
    fn init_with_cpu_status(&mut self, status: &InitialCpuStatus);
}
