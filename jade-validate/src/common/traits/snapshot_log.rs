use crate::common::types::CpuSnapshot;

pub trait SnapshotLog {
    fn create_status_snapshot(&self) -> CpuSnapshot;
}
