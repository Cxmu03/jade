use crate::common::traits::*;
use crate::common::types::*;
use jade::{
    bus::TestBus,
    cpu::{status_flags::StatusFlags, Cpu},
};
use std::fs::File;

pub struct Jade {
    cpu: Cpu<TestBus>,
    bus: TestBus,
}

impl StepCycle for Jade {
    fn step_cycle(&mut self) -> Result<(), ExecutionError> {
        self.step_cycle(&mut self.bus);
    }
}

impl InitializeWithCpuStatus for Jade {
    fn new() -> Self {
        let bus = TestBus::new();
        let cpu = Cpu::new();

        Jade { cpu, bus }
    }

    fn init_with_cpu_status(&mut self, status: &CpuSnapshot) {
        self.cpu.a = status.snapshot.a;
        self.cpu.x = status.snapshot.x;
        self.cpu.y = status.snapshot.y;
        self.cpu.p = StatusFlags(status.snapshot.p);
        self.cpu.sp = status.snapshot.sp;
        self.cpu.db = status.snapshot.db;
        self.cpu.ab = status.snapshot.ab;
        self.cpu.pc = status.snapshot.pc;
        self.cpu.r = status.snapshot.r;
    }
}

impl SnapshotLog for Jade {
    fn create_status_snapshot(&self) -> CpuSnapshot {
        CpuSnapshot {
            a: self.cpu.a,
            x: self.cpu.x,
            y: self.cpu.y,
            p: self.cpu.p.0,
            sp: self.cpu.sp,
            db: self.cpu.db,
            ab: self.cpu.ab,
            pc: self.cpu.pc,
            r: self.cpu.r,
        }
    }
}

impl LoadExecutable for Jade {
    fn load_executable_to(
        &mut self,
        executable: &[u8],
        address: u16,
    ) -> Result<(), ExecutableError> {
        let start = address as usize;
        let end = start + executable.len();
        let overflow = end - MEMORY_SIZE;

        if end > MEMORY_SIZE {
            return Err(ExecutableError::TooLarge(overflow));
        }

        bus.data[start..end].copy_from_slice(executable);

        Ok(())
    }

    fn load_executable_from_file(
        &mut self,
        file: &mut File,
        address: u16,
    ) -> Result<(), ExecutableError> {
        let size = file.metadata()?.len();
        let mut bytes = vec![0u8; size as usize];
        file.read(bytes.as_mut_slice())?;
        self.load_executable_to(bytes.as_slice(), address)?;

        Ok(())
    }
}
