mod programs;

pub use programs::*;

pub trait JadeProgram {
    fn get_start_address() -> u16;

    fn get_executable() -> &'static [u8];
}
