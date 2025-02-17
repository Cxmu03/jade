use super::HasInitialCpuStatus;
use super::SnapshotLog;

pub(crate) trait Validator: HasInitialCpuStatus + SnapshotLog {}

impl<V: HasInitialCpuStatus + SnapshotLog> Validator for V {}
