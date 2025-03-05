use crate::common::traits::*;
use crate::common::types::*;
use jade::{
    bus::{Bus, TestBus},
    cpu::{instruction::CycleType, status_flags::StatusFlags, Cpu},
};
use std::{
    fs::File,
    io::{Read, Write},
};

pub const MEMORY_SIZE: usize = 1 << 16;

pub struct Jade {
    pub cpu: Cpu<TestBus>,
    pub bus: TestBus,
}

impl std::fmt::Debug for Jade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f,
                "cycle: {:2}, a: {:02x} x: {:02x}, y: {:02x}, ab: {:04x}, db: {:02x}, r: {:?}, pc: {:04x}, sp: {:02x}, {:?}, {:?}, {}, p: {}, res: {}",
                self.cpu.cycles - 1, self.cpu.a, self.cpu.x, self.cpu.y, self.cpu.ab, self.cpu.db, self.cpu.r, self.cpu.pc, self.cpu.sp, self.cpu.fetch, self.cpu.execute, self.cpu.execution_state, self.cpu.p, self.cpu.reset
        )
    }
}

impl StepCycle for Jade {
    fn step_cycle(&mut self) -> Result<CpuSnapshot, ExecutionError> {
        self.cpu.step_cycle(&mut self.bus);

        Ok(self.create_status_snapshot())
    }
}

impl DumpMemory for Jade {
    fn dump_memory(&self, file: &mut File) -> Result<usize, std::io::Error> {
        Ok(file.write(&self.bus.data)?)
    }
}

impl InitializeWithCpuStatus for Jade {
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

impl Init for Jade {
    fn new() -> Self {
        let bus = TestBus::new();
        let cpu = Cpu::new();

        Jade { cpu, bus }
    }
}

impl HasInitialCpuStatus for Jade {
    fn reset(&mut self) -> Result<(CpuSnapshot, u16), ExecutionError> {
        self.cpu.reset(&mut self.bus);

        Ok((self.create_status_snapshot(), self.cpu.next_pc))
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

        if end > MEMORY_SIZE {
            let overflow = end - MEMORY_SIZE;
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

    fn set_reset_vector(&mut self, address: u16) {
        let [hi, lo] = address.to_be_bytes();

        self.bus.data[0xfffc] = lo;
        self.bus.data[0xfffd] = hi;
    }
}
