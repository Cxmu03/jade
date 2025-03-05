use crate::common::types::*;
use std::fs::File;

pub trait Init {
    fn new() -> Self
    where
        Self: Sized;
}

pub trait HasInitialCpuStatus {
    fn reset(&mut self) -> Result<(CpuSnapshot, u16), ExecutionError>;
}

pub trait InitializeWithCpuStatus {
    fn init_with_cpu_status(&mut self, status: &CpuSnapshot, new_pc: u16);
}

pub trait LoadExecutable {
    fn load_executable_to(
        &mut self,
        executable: &[u8],
        address: u16,
    ) -> Result<(), ExecutableError>;

    fn load_executable_from_file(
        &mut self,
        file: &mut File,
        address: u16,
    ) -> Result<(), ExecutableError>;

    fn set_reset_vector(&mut self, reset_vector: u16);
}

pub trait SnapshotLog {
    fn create_status_snapshot(&self) -> CpuSnapshot;
}

pub trait StepCycle {
    fn step_cycle(&mut self) -> Result<CpuSnapshot, ExecutionError>;
}

pub trait DumpMemory {
    fn dump_memory(&self, file: &mut File) -> Result<usize, std::io::Error>;
}

pub trait Generator:
    InitializeWithCpuStatus + SnapshotLog + StepCycle + LoadExecutable + Init
{
}

impl<G: InitializeWithCpuStatus + SnapshotLog + StepCycle + LoadExecutable + Init> Generator for G {}

pub trait Validator: HasInitialCpuStatus + SnapshotLog + StepCycle + LoadExecutable + Init {}

impl<V: HasInitialCpuStatus + SnapshotLog + StepCycle + LoadExecutable + Init> Validator for V {}
