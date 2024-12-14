use crate::bus::Bus;

mod instruction;
mod instruction_table;

#[derive(Debug)]
pub struct Cpu {
    bus: Bus,
    pc: u16,
    sp: u8,
    a: u8,
    x: u8,
    y: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            bus: Bus::new(),
            pc: 0,
            sp: 0xFF,
            a: 0,
            x: 0,
            y: 0,
        }
    }

    pub fn step_cycle(&mut self) {
        todo!();
    }

    pub fn step_instruction(&mut self) {
        todo!();
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        self.bus.read_u8(address)
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        u16::from_le_bytes([self.read_u8(address), self.read_u8(address + 1)])
    }
}
