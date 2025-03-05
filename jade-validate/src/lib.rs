pub mod cli;
pub mod common;
pub mod emulators;

use crate::common::{traits::*, types::*};
use jade_programs::*;
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
        }
    }
}

pub fn validate(
    generator: &mut Box<dyn Generator>,
    validator: &mut Box<dyn Validator>,
    program: &Box<dyn JadeProgram>,
    cycles: usize,
) -> ValidationErrorCount {
    let mut error_map = ValidationErrorCount::new();

    let executable = program.get_executable();
    let start_address = program.get_start_address();

    validator
        .load_executable_to(&executable, start_address)
        .unwrap();
    generator
        .load_executable_to(&executable, start_address)
        .unwrap();

    validator.set_reset_vector(start_address);
    generator.set_reset_vector(start_address);

    let (snapshot, new_pc) = validator.reset().unwrap();
    generator.init_with_cpu_status(&snapshot, new_pc);

    for _ in 0..cycles {
        let generator_snapshot = generator.step_cycle().unwrap();
        let validator_snapshot = validator.step_cycle().unwrap();

        validator_snapshot.count_errors(&generator_snapshot, &mut error_map);
    }

    error_map
}
