use crate::common::traits::{HasInitialCpuStatus, LoadExecutable, StepCycle};
use crate::common::types::{ExecutableError, ExecutionError};
use crate::cpu_status::InitialCpuStatus;
use bindings::*;
use core::ffi::c_void;
use std::{fs::File, io::Read, ops::Drop, ptr};

pub mod bindings;

const MEMORY_SIZE: usize = 1 << 16;

pub struct Perfect6502 {
    state: *mut c_void,
}

impl Perfect6502 {
    fn new() -> Self {
        unsafe {
            let state = initAndResetChip();

            Self { state }
        }
    }

    fn state_is_null(&self) -> bool {
        self.state.cast_const() == ptr::null::<c_void>()
    }
}

impl Drop for Perfect6502 {
    fn drop(&mut self) {
        if self.state_is_null() {
            return;
        }

        unsafe {
            destroyChip(self.state);
        }
    }
}

impl StepCycle for Perfect6502 {
    fn step_cycle(&mut self) -> Result<(), ExecutionError> {
        if self.state_is_null() {
            return Err(ExecutionError::InvalidState);
        }

        unsafe {
            step(self.state);
        }

        Ok(())
    }
}

impl HasInitialCpuStatus for Perfect6502 {
    fn get_default_cpu_status(&self) -> crate::cpu_status::InitialCpuStatus {
        todo!()
    }
}

impl LoadExecutable for Perfect6502 {
    fn load_executable_to(
        &mut self,
        executable: &[u8],
        address: u16,
    ) -> Result<(), ExecutableError> {
        let start = address as usize;
        let end = start + executable.len();
        let overflow = end - MEMORY_SIZE;

        if end > MEMORY_SIZE {
            return Err(ExecutableError::TooLarge(overflow));
        }

        unsafe {
            // :(
            memory[start..end].copy_from_slice(executable);
        }

        Ok(())
    }

    fn load_executable_from_file(
        &mut self,
        file: &mut File,
        address: u16,
    ) -> Result<(), ExecutableError> {
        let size = file.metadata()?.len();
        let mut bytes = vec![0u8; size as usize];
        file.read(bytes.as_mut_slice())?;
        self.load_executable_to(bytes.as_slice(), address)?;

        Ok(())
    }
}
