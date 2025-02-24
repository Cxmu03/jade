use super::HasInitialCpuStatus;
use super::Init;
use super::LoadExecutable;
use super::SnapshotLog;
use super::StepCycle;

pub trait Validator: HasInitialCpuStatus + SnapshotLog + StepCycle + LoadExecutable + Init {}

impl<V: HasInitialCpuStatus + SnapshotLog + StepCycle + LoadExecutable + Init> Validator for V {}
