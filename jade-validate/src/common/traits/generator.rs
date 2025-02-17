use super::InitializeWithCpuStatus;
use super::SnapshotLog;
use super::StepCycle;

pub(crate) trait Generator: InitializeWithCpuStatus + SnapshotLog + StepCycle {}

impl<G: InitializeWithCpuStatus + SnapshotLog + StepCycle> Generator for G {}
