use super::bus::Bus;
use instruction::{
    CycleType::{self, *},
    Instruction, InstructionCycle,
};
use instruction_table::INSTRUCTIONS;
use status_flags::StatusFlags;

use strum_macros::Display;

mod instruction;
pub mod instruction_table;
mod microcode_execution;
pub mod status_flags;
#[cfg(test)]
mod tests;

const PAGE_SIZE: u16 = 256;

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
    pub db: u8,       // data bus
    pub ab: u16,      // address bus
    pub r: CycleType, // read/write

    // Registers
    pub pc: u16,        // program counter
    pub sp: u8,         // stack pointer
    pub a: u8,          // accumulator
    pub x: u8,          // x index register
    pub y: u8,          // y index register
    pub p: StatusFlags, // Processor status register

    pub cycles: usize,

    // Debug stuff
    pub execute: Option<InstructionCycle>,
    pub fetch: Option<&'static str>, // TODO: replace &str with enum

    // Misc state machine stuff
    pub execution_state: ExecutionState,
    next_execution_state: ExecutionState,
    on_next_cycle: Option<fn(&mut Self)>,
    pub next_pc: u16,
    pub current_instr: Option<&'static Instruction>,
    // TODO(maybe): replace with iterator solution
    pub current_instr_step: usize,
    pub current_instr_len: usize,
    buf: u8, // Buffer to be used by various microcode steps
    buf16: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            bus: Bus::new(),
            db: 0,
            ab: 0,
            r: ReadCycle,
            pc: 0,
            sp: 0xFD,
            a: 0,
            x: 0,
            y: 0,
            p: StatusFlags::default(),
            cycles: 0,
            execute: None,
            fetch: None,
            execution_state: ExecutionState::Fetch,
            next_execution_state: ExecutionState::Fetch, // This execution state combination is only valid on init
            on_next_cycle: None,
            next_pc: 0,
            current_instr: None,
            current_instr_step: 0,
            current_instr_len: 0,
            buf: 0,
            buf16: 0,
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
        self.current_instr_len = fetched_instruction.cycles.len();
        self.next_pc = self.pc.wrapping_add(1);
        self.r = ReadCycle;

        fetched_instruction
    }

    fn skip_next_cycle(&mut self) {
        self.current_instr_step += 1;
    }

    fn end_instruction(&mut self) {
        self.current_instr_step = self.current_instruction_len() - 1;
    }

    fn end_instruction_if(&mut self, condition: bool) {
        if condition {
            self.end_instruction();
        }
    }

    fn add_with_carry<const SET_OVERFLOW: bool>(
        &mut self,
        operand1: u8,
        operand2: u8,
        carry: bool,
    ) -> u8 {
        let op1_before = operand1;
        let carry = carry as u8;
        let result = self.a.wrapping_add(operand2).wrapping_add(carry);
        let result_u16 = u16::from(operand1) + u16::from(operand2) + u16::from(carry);

        self.p.set_c(result_u16 > 0xFF);
        self.update_zero_negative_flags(result);

        if SET_OVERFLOW {
            let did_overflow = ((op1_before ^ result) & (operand2 ^ result) & 0x80) == 0x80;
            self.p.set_v(did_overflow);
        }

        result
    }

    fn compare(&mut self, operand1: u8, operand2: u8) {
        self.add_with_carry::<false>(operand1, !operand2, true);
    }

    fn add_offset_to_address<T: Into<i16>>(address: u16, operand: T) -> (bool, u16, u16) {
        let [page, _] = address.to_be_bytes();
        let new_address = address.wrapping_add(Into::<i16>::into(operand) as u16);
        let [new_page, new_offset] = new_address.to_be_bytes();

        let new_partial_address = u16::from_be_bytes([page, new_offset]);

        (new_page != page, new_partial_address, new_address)
    }

    fn process_indexed_operand<const SKIP_ON_PAGE_CROSS: bool>(
        &mut self,
        register: u8,
    ) -> (CycleType, u16) {
        let hi = self.db;
        let lo = self.buf;
        let address = u16::from_be_bytes([hi, lo]);

        let (page_crossed, new_partial_address, new_address) =
            Self::add_offset_to_address(address, register);

        self.buf16 = new_address;
        self.ab = new_partial_address;
        self.read_memory();

        if SKIP_ON_PAGE_CROSS && !page_crossed {
            self.skip_next_cycle();
        }

        (ReadCycle, self.pc)
    }

    fn push_stack(&mut self) {
        self.ab = u16::from_be_bytes([0x01, self.sp]);
        self.write_memory();
        self.sp = self.sp.wrapping_sub(1);
    }

    fn pop_stack(&mut self) {
        self.sp = self.sp.wrapping_add(1);
        self.ab = u16::from_be_bytes([0x01, self.sp]);
        self.read_memory();
    }

    fn add_offset_to_stack_address(address: u16, offset: u8) -> u16 {
        let [hi, mut lo] = address.to_be_bytes();
        lo = lo.wrapping_add(offset);

        u16::from_be_bytes([hi, lo])
    }

    // Note: in the current state machine model, the state of the step_cycle call that was last executed will be saved
    // into execution_state of the CPU. Furthermore, the execution state FetchExecute will only be set after executing
    // a microcode step due to how cycles are marked as read/write cycles. As the state following a FetchExecute will
    // always be an Execute state and this will replace the execution_state on the next step_cycle call, FetchExecute
    // can never be a valid execution state at the start of step_cycle after setting the state
    pub fn step_cycle(&mut self) {
        self.execution_state = self.next_execution_state;
        self.pc = self.next_pc;

        if let Some(fun) = self.on_next_cycle.take() {
            fun(self);
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

                if self.current_instr_step == self.current_instr_len {
                    if self.r == ReadCycle {
                        let fetched_instruction = self.fetch_instruction();
                        self.fetch = Some(fetched_instruction.identifier);
                        self.execution_state = ExecutionState::FetchExecute;
                        self.next_execution_state = ExecutionState::Execute;
                    } else {
                        self.next_execution_state = ExecutionState::Fetch;
                    }
                }
            }
            ExecutionState::FetchExecute => {
                unreachable!("This should not be possible with internal control flow :(")
            }
        };

        self.cycles += 1;
    }

    pub fn step_instruction(&mut self) {
        self.step_cycle();

        while self.next_execution_state != ExecutionState::Fetch
            && self.execution_state != ExecutionState::FetchExecute
        {
            self.step_cycle();
        }
    }

    pub fn read_memory(&mut self) {
        self.db = self.bus.read_u8(self.ab);
    }

    pub fn write_memory(&mut self) {
        self.bus.write_u8(self.ab, self.db);
    }
}

#[cfg(test)]
mod unit_test {
    use super::Cpu;

    #[test]
    fn add_offset_to_address_no_page_cross() {
        let address: u16 = 0x1000;
        let offset = 0xf0u8;

        let (page_crossed, new_partial_address, new_address) =
            Cpu::add_offset_to_address(address, offset);

        assert_eq!(page_crossed, false);
        assert_eq!(new_partial_address, new_address);
        assert_eq!(new_address, 0x10f0);
    }

    #[test]
    fn add_offset_to_address_page_cross() {
        let address: u16 = 0x1005;
        let offset = 0xffu8;

        let (page_crossed, new_partial_address, new_address) =
            Cpu::add_offset_to_address(address, offset);

        assert_eq!(page_crossed, true);
        assert_eq!(new_partial_address, 0x1004);
        assert_eq!(new_address, 0x1104);
    }

    #[test]
    fn add_offset_to_address_page_cross_overflow() {
        let address: u16 = 0xfffe;
        let offset = 0x3u8;

        let (page_crossed, new_partial_address, new_address) =
            Cpu::add_offset_to_address(address, offset);

        assert_eq!(page_crossed, true);
        assert_eq!(new_partial_address, 0xff01);
        assert_eq!(new_address, 0x0001);
    }

    #[test]
    fn add_offset_to_address_page_cross_underflow() {
        let address: u16 = 0x0003;
        let offset = 0xF0u8 as i8;

        let (page_crossed, new_partial_address, new_address) =
            Cpu::add_offset_to_address(address, offset);

        assert_eq!(page_crossed, true);
        assert_eq!(new_partial_address, 0x00f3);
        assert_eq!(new_address, 0xfff3);
    }
}
