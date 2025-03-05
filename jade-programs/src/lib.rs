mod programs;

pub use programs::*;

pub trait JadeProgram {
    fn get_start_address(&self) -> u16;

    fn get_executable(&self) -> &'static [u8];
}
