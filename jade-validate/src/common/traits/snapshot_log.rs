use crate::common::types::CpuSnapshot;

pub(crate) trait SnapshotLog {
    fn create_status_snapshot(&self) -> CpuSnapshot;
}
