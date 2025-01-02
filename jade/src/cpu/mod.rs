use super::bus::Bus;
use instruction::{CycleType::*, Instruction, InstructionCycle, InstructionCycle::*};
use instruction_table::INSTRUCTIONS;
use strum_macros::Display;

mod instruction;
pub mod instruction_table;

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
    pub pc: u16, // program counter
    pub sp: u8,  // stack pointer
    pub a: u8,   // accumulator
    pub x: u8,   // x index register
    pub y: u8,   // y index register

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

    pub fn execute_microcode_step(&mut self) -> InstructionCycle {
        let step: InstructionCycle = self.current_instr.unwrap().cycles[self.current_instr_step];

        let (cycle_type, next_pc) = match step {
            Read => {
                self.ab = self.pc;
                self.read_memory();

                (ReadCycle, self.pc)
            }
            ImmOperand => {
                self.ab = self.pc;
                self.read_memory();

                (ReadCycle, self.pc + 1)
            }
            ZpgOperand => {
                self.ab = self.pc;
                self.read_memory();

                (ReadCycle, self.pc + 1)
            }
            Jsr1 => {
                self.ab = self.pc;
                self.read_memory();

                (ReadCycle, self.pc + 1)
            }
            Jsr2 => {
                self.ab = 1 * PAGE_SIZE + u16::from(self.sp);
                // For some reason, the 6502 uses the stack pointer register to buffer the lower address byte,
                // which is kinda insane.
                self.sp = self.db;
                // Is basically a dummy read cycle to buffer the lower operand byte but read anyway to be
                // compatible with simulators
                self.read_memory();

                (ReadCycle, self.pc)
            }
            Jsr3 => {
                self.db = (self.pc >> 8) as u8;
                self.write_memory(); // pc_h

                (WriteCycle, self.pc)
            }
            Jsr4 => {
                self.ab -= 1;
                // Store lower part of ab (real stack pointer) to restore it later
                self.buf = self.ab as u8;
                self.db = self.pc as u8;
                self.write_memory(); // pc_l

                (WriteCycle, self.pc)
            }
            Jsr5 => {
                self.ab = self.pc;
                self.read_memory(); // op_h

                (ReadCycle, self.pc + 1)
            }
            Jsr6 => {
                self.ab = u16::from_le_bytes([self.sp, self.db]);
                self.sp = self.buf;
                self.pc = self.ab;

                (ReadCycle, self.ab)
            }
            Lda => {
                self.a = self.db;

                (ReadCycle, self.pc + 1)
            }
            Inx2 => {
                self.ab = self.pc;
                self.read_memory();

                // This is necessary because although the incremented x is already on the special bus, the control signal
                // to transfer sb to X (SBX or dpc3_SBX) will only fire on phi1 of the next cycle
                self.on_next_cycle = Some(|cpu| {
                    cpu.x = cpu.x.wrapping_add(1);
                });

                (ReadCycle, self.pc)
            }
            Dey2 => {
                self.ab = self.pc;
                self.read_memory();

                self.on_next_cycle = Some(|cpu| {
                    cpu.y = cpu.y.wrapping_sub(1);
                });

                (ReadCycle, self.pc)
            }
            Inc2 => {
                self.ab = self.db as u16;
                self.read_memory();

                (ReadCycle, self.pc)
            }
            Inc3 => {
                self.write_memory();

                (WriteCycle, self.pc)
            }
            Inc4 => {
                self.db = u8::wrapping_add(self.db, 1);
                self.write_memory();

                (WriteCycle, self.pc)
            }
            NYI => panic!(
                "Instruction {} is not yet implemented",
                self.current_instr.unwrap().identifier
            ),
        };

        self.r = cycle_type.into();
        self.current_instr_step += 1;
        self.next_pc = next_pc;

        step
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
