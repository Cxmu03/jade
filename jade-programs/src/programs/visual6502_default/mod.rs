use crate::JadeProgram;

const VISUAL_6502_DEFAULT_EXECUTABLE: &[u8] = &[
    0xa9, 0x00, 0x20, 0x10, 0x00, 0x4c, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x40, 0xe8, 0x88, 0xe6, 0x0f, 0x38, 0x69, 0x02, 0x60,
];

/// The default program of the visual 6502
#[derive(Debug, Clone)]
pub struct Visual6502Default;

impl Visual6502Default {
    pub fn new() -> Self {
        Self {}
    }
}

impl JadeProgram for Visual6502Default {
    fn get_start_address(&self) -> u16 {
        0x0000
    }

    fn get_load_address(&self) -> u16 {
        0x0000
    }

    fn get_executable(&self) -> &'static [u8] {
        &VISUAL_6502_DEFAULT_EXECUTABLE
    }

    fn get_name(&self) -> &str {
        "Visual6502 Default"
    }
}

