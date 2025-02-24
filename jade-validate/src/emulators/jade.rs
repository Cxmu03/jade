use crate::common::traits::*;
use crate::common::types::*;
use jade::{
    bus::{Bus, TestBus},
    cpu::{instruction::CycleType, status_flags::StatusFlags, Cpu},
};
use std::{fs::File, io::Read};

pub const MEMORY_SIZE: usize = 1 << 16;

pub struct Jade {
    pub cpu: Cpu<TestBus>,
    bus: TestBus,
}

impl StepCycle for Jade {
    fn step_cycle(&mut self) -> Result<CpuSnapshot, ExecutionError> {
        self.cpu.step_cycle(&mut self.bus);

        Ok(self.create_status_snapshot())
    }
}

impl InitializeWithCpuStatus for Jade {
    fn new() -> Self {
        let bus = TestBus::new();
        let cpu = Cpu::new();

        Jade { cpu, bus }
    }

    fn init_with_cpu_status(&mut self, snapshot: &CpuSnapshot, new_pc: u16) {
        self.cpu.a = snapshot.a;
        self.cpu.x = snapshot.x;
        self.cpu.y = snapshot.y;
        self.cpu.p = StatusFlags(snapshot.p);
        self.cpu.sp = snapshot.sp;
        self.cpu.db = snapshot.db;
        self.cpu.ab = snapshot.ab;
        self.cpu.pc = snapshot.pc;
        self.cpu.next_pc = new_pc;
        self.cpu.r = if snapshot.r {
            CycleType::ReadCycle
        } else {
            CycleType::WriteCycle
        };
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
            r: self.cpu.r == CycleType::ReadCycle,
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

        self.bus.data[start..end].copy_from_slice(executable);

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
