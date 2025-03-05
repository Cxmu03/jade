use std::{collections::HashMap, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("The internal emulator state is invalid")]
    InvalidState,
}

#[derive(Error, Debug)]
pub enum ExecutableError {
    #[error("The executable does not fit into the remaining memory ({0} bytes too large)")]
    TooLarge(usize),
    #[error("Executable could not be read from file")]
    InvalidFile(#[from] io::Error),
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum ValidationError {
    ControlFlow,
    Register,
    Io,
    Status,
}

pub struct ValidationErrorCount {
    error_map: HashMap<ValidationError, usize>,
}

impl ValidationErrorCount {
    pub fn new() -> Self {
        use super::ValidationError::*;

        Self {
            error_map: HashMap::from([(ControlFlow, 0), (Register, 0), (Io, 0), (Status, 0)]),
        }
    }

    pub fn increment_count_of(&mut self, error_type: ValidationError) {
        self.error_map
            .entry(error_type)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    pub fn get_count_of(&self, error_type: ValidationError) -> usize {
        self.error_map.get(&error_type).unwrap_or(&0).to_owned()
    }
}
