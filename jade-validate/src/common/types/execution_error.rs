use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("The internal emulator state is invalid")]
    InvalidState,
}
