use super::Init;
use super::InitializeWithCpuStatus;
use super::LoadExecutable;
use super::SnapshotLog;
use super::StepCycle;

pub trait Generator:
    InitializeWithCpuStatus + SnapshotLog + StepCycle + LoadExecutable + Init
{
}

impl<G: InitializeWithCpuStatus + SnapshotLog + StepCycle + LoadExecutable + Init> Generator for G {}
