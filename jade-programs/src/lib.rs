mod programs;

pub use programs::*;

pub trait HasStartAddress {
    fn get_start_address() -> u16;
}

pub trait HasExecutable {
    fn get_executable() -> &'static [u8];
}
