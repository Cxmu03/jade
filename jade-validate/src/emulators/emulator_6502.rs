// Wrapper around the emulator_6502 crate to make it jade_validate compatible

use emulator_6502::{Interface6502, MOS6502};
use jade::bus::{Bus, TestBus};

use crate::common::traits::{SnapshotLog, StepCycle};
use crate::common::types::{CpuSnapshot, ExecutionError};

pub struct Mos6502Bus(TestBus);

impl Mos6502Bus {
    pub fn load_program(&mut self, program: &[u8], address: u16) {
        let start = address as usize;
        let end = start + program.len();

        self.0.data[start..end].copy_from_slice(program);
    }
}

impl Interface6502 for Mos6502Bus {
    fn read(&mut self, address: u16) -> u8 {
        self.0.read_u8(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.0.write_u8(address, value);
    }
}

pub struct Emulator6502 {
    pub cpu: MOS6502,
    pub bus: Mos6502Bus,
}

impl Emulator6502 {
    pub fn new() -> Self {
        Emulator6502 {
            cpu: MOS6502::new(),
            bus: Mos6502Bus(TestBus::new()),
        }
    }
}

impl SnapshotLog for Emulator6502 {
    fn create_status_snapshot(&self) -> CpuSnapshot {
        CpuSnapshot {
            a: self.cpu.get_accumulator(),
            x: self.cpu.get_x_register(),
            y: self.cpu.get_y_register(),
            p: self.cpu.get_status_register(),
            sp: self.cpu.get_stack_pointer(),
            db: 0,
            ab: 0,
            pc: self.cpu.get_program_counter(),
            r: true,
        }
    }
}

impl StepCycle for Emulator6502 {
    fn step_cycle(&mut self) -> Result<CpuSnapshot, ExecutionError> {
        self.cpu.cycle(&mut self.bus);

        Ok(self.create_status_snapshot())
    }
}
