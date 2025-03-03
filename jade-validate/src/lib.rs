pub mod common;
pub mod emulators;

use crate::common::{traits::*, types::*};
use jade_programs::*;
use strum::EnumString;

#[derive(Debug, EnumString)]
pub enum Generators {
    #[strum(ascii_case_insensitive)]
    Jade,
}

#[derive(Debug, EnumString)]
pub enum Validators {
    #[strum(ascii_case_insensitive, serialize = "perfect")]
    Perfect6502,
}

impl Generators {
    fn new_generator(&self) -> Box<dyn Generator> {
        match self {
            Generators::Jade => Box::new(emulators::Jade::new()),
        }
    }
}

impl Validators {
    fn new_validator(&self) -> Box<dyn Validator> {
        match self {
            Validators::Perfect6502 => Box::new(emulators::Perfect6502::new()),
        }
    }
}

pub fn validate(generator: impl Generator, validator: impl Validator, program: impl JadeProgram) {}
