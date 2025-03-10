use crate::JadeProgram;
use lazy_static_include::*;

lazy_static_include_bytes! {
    DORMANN_EXECUTABLE => "src/programs/dormann/6502_functional_test.bin"
}

/// The functional test suite written by Klaus Dormann
/// Takes about 90M cycles to complete
#[derive(Debug, Clone)]
pub struct Dormann;

impl Dormann {
    pub fn new() -> Self {
        Dormann {}
    }
}

impl JadeProgram for Dormann {
    fn get_start_address(&self) -> u16 {
        0x0400
    }

    fn get_load_address(&self) -> u16 {
        0x00
    }

    fn get_executable(&self) -> &'static [u8] {
        &DORMANN_EXECUTABLE
    }

    fn get_name(&self) -> &str {
        "Dormann"
    }
}
