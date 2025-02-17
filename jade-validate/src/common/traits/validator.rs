use super::HasInitialCpuStatus;
use super::SnapshotLog;
use super::StepCycle;

pub(crate) trait Validator: HasInitialCpuStatus + SnapshotLog + StepCycle {}

impl<V: HasInitialCpuStatus + SnapshotLog + StepCycle> Validator for V {}
