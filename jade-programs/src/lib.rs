mod programs;

pub use programs::*;
use std::str::FromStr;
use thiserror::Error;

pub trait JadeProgram {
    fn get_start_address(&self) -> u16;

    fn get_executable(&self) -> &[u8];

    fn get_name(&self) -> &str;
}

pub struct GenericJadeProgram {
    pub executable: Box<[u8]>,
    pub start_addr: u16,
    pub name: String,
}

impl JadeProgram for GenericJadeProgram {
    fn get_executable(&self) -> &[u8] {
        &self.executable
    }

    fn get_start_address(&self) -> u16 {
        self.start_addr
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Error, Debug)]
pub enum JadeProgramParseError {
    #[error("{0} is not a valid builtin name")]
    InvalidName(String),
}

impl FromStr for Box<dyn JadeProgram> {
    type Err = JadeProgramParseError;

    fn from_str(name: &str) -> Result<Box<dyn JadeProgram>, Self::Err> {
        match name.to_lowercase().as_str() {
            "md5" => Ok(Box::new(Md5::new())),
            _ => Err(JadeProgramParseError::InvalidName(name.to_owned())),
        }
    }
}
