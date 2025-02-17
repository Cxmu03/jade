use super::InitializeWithCpuStatus;
use super::SnapshotLog;

pub(crate) trait Generator: InitializeWithCpuStatus + SnapshotLog {}

impl<G: InitializeWithCpuStatus + SnapshotLog> Generator for G {}
