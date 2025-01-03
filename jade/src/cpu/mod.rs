use super::bus::Bus;
use instruction::{CycleType::*, Instruction, InstructionCycle, InstructionCycle::*};
use instruction_table::INSTRUCTIONS;
use status_flags::StatusFlags;

use strum_macros::Display;

mod instruction;
pub mod instruction_table;
mod microcode_execution;
pub mod status_flags;

const PAGE_SIZE: u16 = 256;

// TODO(maybe): have variants contain data about fetched instruction or cycle identifier
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq)]
pub enum ExecutionState {
    Fetch,
    Execute,
    FetchExecute,
}

#[derive(Debug)]
pub struct Cpu {
    pub bus: Bus,

    // Outputs/Inputs
    pub db: u8,  // data bus
    pub ab: u16, // address bus
    pub r: u8,   // read/write

    // Registers
    pub pc: u16,        // program counter
    pub sp: u8,         // stack pointer
    pub a: u8,          // accumulator
    pub x: u8,          // x index register
    pub y: u8,          // y index register
    pub p: StatusFlags, // Processor status register

    // Debug stuff
    pub execute: Option<InstructionCycle>,
    pub fetch: Option<&'static str>, // TODO: replace &str with enum

    // Misc state machine stuff
    pub execution_state: ExecutionState,
    next_execution_state: ExecutionState,
    on_next_cycle: Option<fn(&mut Self) -> ()>,
    pub next_pc: u16,
    // TODO: replace with reference to instruction
    pub current_instr: Option<&'static Instruction>,
    // TODO(maybe): replace with iterator solution
    pub current_instr_step: usize, // The current cycle index of the instruction
    buf: u8,                       // Buffer to be used by various microcode steps
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
            p: StatusFlags::default(),
            execute: None,
            fetch: None,
            execution_state: ExecutionState::Fetch,
            next_execution_state: ExecutionState::Fetch, // This execution state combination is only valid on init
            on_next_cycle: None,
            next_pc: 0,
            current_instr: None,
            current_instr_step: 0,
            buf: 0,
        }
    }

    fn update_zero_negative_flags(&mut self, value: u8) {
        self.p.set_z(value == 0);
        self.p.set_n(value >> 7 == 1)
    }

    fn load_a(&mut self, value: u8) {
        self.a = value;
        self.update_zero_negative_flags(self.a);
    }

    fn load_x(&mut self, value: u8) {
        self.x = value;
        self.update_zero_negative_flags(self.x);
    }

    fn load_y(&mut self, value: u8) {
        self.y = value;
        self.update_zero_negative_flags(self.y);
    }

    pub fn current_instruction_len(&self) -> usize {
        self.current_instr.unwrap().cycles.len()
    }

    pub fn fetch_instruction(&mut self) -> &Instruction {
        self.ab = self.pc;
        self.read_memory();

        let fetched_instruction: &Instruction = &INSTRUCTIONS[self.db as usize];
        self.current_instr = Some(fetched_instruction);
        self.current_instr_step = 0;
        self.next_pc = self.pc + 1;

        fetched_instruction
    }

    // Note: in the current state machine model, the state of the step_cycle call that was last executed will be saved
    // into execution_state of the CPU. Furthermore, the execution state FetchExecute will only be set after executing
    // a microcode step due to how cycles are marked as read/write cycles. As the state following a FetchExecute will
    // always be an Execute state and this will replace the execution_state on the next step_cycle call, FetchExecute
    // can never be a valid execution state at the start of step_cycle after setting the state
    pub fn step_cycle(&mut self) {
        self.execution_state = self.next_execution_state;
        self.pc = self.next_pc;

        if let Some(fun) = self.on_next_cycle {
            fun(self);
            self.on_next_cycle = None;
        }

        match self.execution_state {
            ExecutionState::Fetch => {
                let fetched_instruction = self.fetch_instruction();
                self.fetch = Some(fetched_instruction.identifier);
                self.execute = None;
                self.next_execution_state = ExecutionState::Execute;
            }
            ExecutionState::Execute => {
                let executed_step = self.execute_microcode_step();
                self.fetch = None;
                self.execute = Some(executed_step);

                if self.current_instr_step == self.current_instruction_len() {
                    if self.r == ReadCycle.into() {
                        let fetched_instruction = self.fetch_instruction();
                        self.fetch = Some(fetched_instruction.identifier);
                        self.execution_state = ExecutionState::FetchExecute;
                        self.next_execution_state = ExecutionState::Execute;
                    } else {
                        self.next_execution_state = ExecutionState::Fetch;
                    }
                }
            }
            // TODO(maybe): Evaluate execution state model
            ExecutionState::FetchExecute => {
                unreachable!("This should not be possible with internal control flow :(")
            }
        }
    }

    pub fn step_instruction(&mut self) {
        todo!();
    }

    pub fn read_memory(&mut self) {
        self.db = self.bus.read_u8(self.ab);
    }

    pub fn write_memory(&mut self) {
        self.bus.write_u8(self.ab, self.db);
    }
}
