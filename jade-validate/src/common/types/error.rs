use std::io;
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

pub enum ValidationError {
    ControlFlow,
    Register,
    Status,
}
