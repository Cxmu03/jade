use crate::common::traits::StepCycle;
use crate::common::types::ExecutionError;
use bindings::*;
use core::ffi::c_void;
use std::{ops::Drop, ptr};

pub mod bindings;

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
        unsafe { self.state.cast_const() == ptr::null::<c_void>() }
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
