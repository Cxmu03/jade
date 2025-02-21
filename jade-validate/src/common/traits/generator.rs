use super::InitializeWithCpuStatus;
use super::LoadExecutable;
use super::SnapshotLog;
use super::StepCycle;

pub(crate) trait Generator:
    InitializeWithCpuStatus + SnapshotLog + StepCycle + LoadExecutable
{
}

impl<G: InitializeWithCpuStatus + SnapshotLog + StepCycle + LoadExecutable> Generator for G {}
