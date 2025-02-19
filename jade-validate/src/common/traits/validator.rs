use super::HasInitialCpuStatus;
use super::LoadExecutable;
use super::SnapshotLog;
use super::StepCycle;

pub(crate) trait Validator:
    HasInitialCpuStatus + SnapshotLog + StepCycle + LoadExecutable
{
}

impl<V: HasInitialCpuStatus + SnapshotLog + StepCycle + LoadExecutable> Validator for V {}
