use crate::cpu_status::CpuStatus;

pub(crate) trait StatusLog {
    fn create_status_snapshot(&self) -> CpuStatus;
}
