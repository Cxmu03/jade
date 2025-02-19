use crate::common::types::ExecutableError;
use std::fs::File;

pub trait LoadExecutable {
    fn load_executable_to(
        &mut self,
        executable: &[u8],
        address: u16,
    ) -> Result<(), ExecutableError>;
    fn load_executable_from_file(
        &mut self,
        file: &mut File,
        address: u16,
    ) -> Result<(), ExecutableError>;
}
