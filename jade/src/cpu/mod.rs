use super::bus::Bus;
use instruction::{CycleType::*, Instruction, InstructionCycle, InstructionCycle::*};
use instruction_table::INSTRUCTIONS;

mod instruction;
mod instruction_table;

#[derive(Debug)]
pub struct Cpu {
    pub bus: Bus,

    // Outputs / Inputs
    pub db: u8,
    pub ab: u16,

    // Registers
    pub pc: u16,
    pub sp: u8,
    pub a: u8,
    pub x: u8,
    pub y: u8,

    // Misc state machine stuff
    pub current_instr: usize, // The instruction_table index of the current instruction
    current_instr_step: usize, // The current cycle index of the instruction
    buf: u8,                  // Buffer to be used by various microcode steps
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            bus: Bus::new(),
            db: 0,
            ab: 0,
            pc: 0,
            sp: 0xFF,
            a: 0,
            x: 0,
            y: 0,
            current_instr: 0,
            current_instr_step: 0,
            buf: 0,
        }
    }

    pub fn execute_microcode_step(&mut self) {
        let instruction: &Instruction = &INSTRUCTIONS[self.current_instr];
        let step: InstructionCycle = instruction.cycles[self.current_instr_step];

        let (cycle_type, new_pc) = match step {
            ImmOperand => {
                self.ab = self.pc;
                self.db = self.read_u8();
                (ReadCycle, self.pc + 1)
            }
            Lda => {
                self.a = self.db;
                (ReadCycle, self.pc + 1)
            }
            NYI => panic!("Instruction {} is not yet implemented", self.current_instr),
        };

        // TODO: Add fetch pipelining

        self.current_instr_step += 1;
        self.pc = new_pc;
    }

    pub fn step_cycle(&mut self) {
        todo!();
    }

    pub fn step_instruction(&mut self) {
        todo!();
    }

    pub fn read_u8(&self) -> u8 {
        self.bus.read_u8(self.ab)
    }

    // Not sure this will be needed
    /*pub fn read_u16(&self) -> u16 {
        u16::from_le_bytes([self.read_u8(), self.read_u8()])
    }*/
}
