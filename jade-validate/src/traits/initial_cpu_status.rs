use crate::cpu_status::InitialCpuStatus;

pub(crate) trait HasInitialCpuStatus {
    fn get_default_cpu_status(&self) -> InitialCpuStatus;
}

pub(crate) trait InitializeWithCpuStatus {
    fn init_with_cpu_status(&mut self, status: &InitialCpuStatus);
}
