use std::io::Read;

use super::bus::Bus;
use instruction::{CycleType::*, Instruction, InstructionCycle, InstructionCycle::*};
use instruction_table::INSTRUCTIONS;

mod instruction;
pub mod instruction_table;

const PAGE_SIZE: u16 = 256;

#[derive(Debug)]
pub struct Cpu {
    pub bus: Bus,

    // Outputs/Inputs
    pub db: u8,  // data bus
    pub ab: u16, // address bus
    pub r: u8,   // read/write

    // Registers
    pub pc: u16, // program counter
    pub sp: u8,  // stack pointer
    pub a: u8,   // accumulator
    pub x: u8,   // x index register
    pub y: u8,   // y index register

    // Misc state machine stuff
    pub current_instr: usize, // The instruction_table index of the current instruction
    pub current_instr_step: usize, // The current cycle index of the instruction
    buf: u8,                  // Buffer to be used by various microcode steps
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            bus: Bus::new(),
            db: 0,
            ab: 0,
            r: ReadCycle.into(),
            pc: 0,
            sp: 0xFD,
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
            Jsr1 => {
                self.ab = self.pc;
                self.db = self.read_u8();

                (ReadCycle, self.pc + 1)
            }
            Jsr2 => {
                self.ab = 1 * PAGE_SIZE + u16::from(self.sp);
                // For some reason, the 6502 uses the stack pointer register to buffer the lower address byte,
                // which is kinda insane.
                self.sp = self.db;
                // Is basically a dummy read cycle to buffer the lower operand byte but read anyway to be
                // compatible with simulators
                self.db = self.read_u8();

                (ReadCycle, self.pc)
            }
            Jsr3 => {
                self.db = (self.pc >> 8) as u8;
                self.write_u8(); // pc_h

                (WriteCycle, self.pc)
            }
            Jsr4 => {
                self.ab -= 1;
                // Store lower part of ab (real stack pointer) to restore it later
                self.buf = self.ab as u8;
                self.db = self.pc as u8;
                self.write_u8(); // pc_l

                (WriteCycle, self.pc)
            }
            Jsr5 => {
                self.ab = self.pc;
                self.db = self.read_u8(); // op_h

                (ReadCycle, self.pc + 1)
            }
            Jsr6 => {
                self.ab = u16::from_le_bytes([self.sp, self.db]);
                self.sp = self.buf;

                (ReadCycle, self.ab)
            }
            Lda => {
                self.a = self.db;

                (ReadCycle, self.pc + 1)
            }
            NYI => panic!("Instruction {} is not yet implemented", self.current_instr),
        };

        // TODO: Add fetch pipelining

        self.r = cycle_type.into();
        self.current_instr_step += 1;

        // TODO: pc should only be updated on fetch or fetch-execute cycles (I think)
        // Until those are implemented, the current implementation leads to correct behavior for jumps
        self.pc = new_pc;
    }

    pub fn step_cycle(&mut self) {
        todo!();
    }

    pub fn step_instruction(&mut self) {
        todo!();
    }

    // TODO: Maybe read_u8 should read directly into the data bus as write_u8 implicitly uses ab
    pub fn read_u8(&self) -> u8 {
        self.bus.read_u8(self.ab)
    }

    pub fn write_u8(&mut self) {
        self.bus.write_u8(self.ab, self.db);
    }
}
