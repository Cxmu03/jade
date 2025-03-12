pub mod cli;
pub mod common;
pub mod emulators;

use crate::common::{traits::*, types::*};
use jade_programs::*;
use ringbuffer::{ConstGenericRingBuffer, RingBuffer};
use strum::EnumString;

#[derive(Debug, Clone, EnumString)]
pub enum GeneratorType {
    #[strum(ascii_case_insensitive)]
    Jade,
}

#[derive(Debug, Clone, EnumString)]
pub enum ValidatorType {
    #[strum(ascii_case_insensitive, serialize = "perfect")]
    Perfect6502,
    #[strum(ascii_case_insensitive)]
    Jade,
}

impl GeneratorType {
    pub fn new_generator(&self) -> Box<dyn Generator> {
        match self {
            GeneratorType::Jade => Box::new(emulators::Jade::new()),
        }
    }
}

impl ValidatorType {
    pub fn new_validator(&self) -> Box<dyn Validator> {
        match self {
            ValidatorType::Perfect6502 => Box::new(emulators::Perfect6502::new()),
            ValidatorType::Jade => Box::new(emulators::Jade::new()),
        }
    }
}

pub fn validate(
    generator: &mut Box<dyn Generator>,
    validator: &mut Box<dyn Validator>,
    program: &Box<dyn JadeProgram>,
    cycles: usize,
) -> ValidationErrorCounter {
    let mut error_map = ValidationErrorCounter::new();

    let executable = program.get_executable();
    let load_address = program.get_load_address();
    let start_address = program.get_start_address();

    validator
        .load_executable_to(&executable, load_address)
        .unwrap();
    generator
        .load_executable_to(&executable, load_address)
        .unwrap();

    validator.set_reset_vector(start_address);
    generator.set_reset_vector(start_address);

    let (snapshot, new_pc) = validator.reset().unwrap();
    generator.init_with_cpu_status(&snapshot, new_pc);

    let mut break_on_next_cycle = false;

    for i in 0..cycles {
        let generator_snapshot = generator.step_cycle().unwrap();
        let validator_snapshot = validator.step_cycle().unwrap();

        let error_count = validator_snapshot.count_errors(&generator_snapshot, &mut error_map);

        if error_count.register > 0
            || error_count.io > 0
            || error_count.control_flow > 0
            || break_on_next_cycle
        {
            println!("{generator:?}\n{generator_snapshot:?}\n{validator_snapshot:?}\n");
            if break_on_next_cycle {
                break;
            }
            break_on_next_cycle = true;
        }

        /*if cycles - i < 50 {
            println!("{generator:?}\n{generator_snapshot:?}\n{validator_snapshot:?}\n");
        }*/
    }

    error_map
}

pub fn run(emulator: &mut Box<dyn Validator>, program: &Box<dyn JadeProgram>, cycles: usize) {
    let mut pc_buffer = ConstGenericRingBuffer::<u16, 6>::new();
    let mut trace_buffer = ConstGenericRingBuffer::<String, 50>::new();
    let executable = program.get_executable();
    let load_address = program.get_load_address();
    let start_address = program.get_start_address();

    emulator
        .load_executable_to(&executable, load_address)
        .unwrap();
    emulator.set_reset_vector(start_address);

    let initial_snapshot = emulator.reset().unwrap();
    println!("0: {:?}", initial_snapshot);

    for i in 1..cycles {
        let snapshot = emulator.step_cycle().unwrap();

        if i % 500000 == 0 {
            println!("{i}");
        }

        if pc_buffer.len() == 6 && snapshot.pc == pc_buffer[0] && snapshot.pc == pc_buffer[3] {
            trace_buffer.into_iter().for_each(|item| println!("{item}"));

            break;
        }

        pc_buffer.push(snapshot.pc);
        trace_buffer.push(format!("{emulator:?}"));
    }
}
