use crate::JadeProgram;
use lazy_static_include::*;

lazy_static_include_bytes! {
    MD5_EXECUTABLE => "src/programs/md5/main"
}

/// A reimplementation of the md5 algorithm for the 6502
/// Takes about 9M cycles to complete
#[derive(Debug, Clone)]
pub struct Md5;

impl Md5 {
    pub fn new() -> Md5 {
        Md5 {}
    }
}

impl JadeProgram for Md5 {
    fn get_start_address(&self) -> u16 {
        0x0200
    }

    fn get_load_address(&self) -> u16 {
        0x0200
    }

    fn get_executable(&self) -> &'static [u8] {
        &MD5_EXECUTABLE
    }

    fn get_name(&self) -> &str {
        "MD5"
    }
}
