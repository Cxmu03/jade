pub trait Bus {
    fn new() -> Self;

    fn read_u8(&self, address: u16) -> u8;

    fn write_u8(&mut self, address: u16, value: u8);
}

/*
    For now this will only contain a stub implementation of the NES memory.
    This is for done only testing the 6502 implementation and will be expanded
    afterwards.
*/
#[derive(Debug)]
pub struct NesBus {
    pub data: [u8; 1 << 16], // TODO: make private when possible
}

impl Bus for NesBus {
    fn new() -> Self {
        NesBus { data: [0; 1 << 16] }
    }

    fn read_u8(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    fn write_u8(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }
}
