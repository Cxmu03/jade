/*
    For now this will only contain a stub implementation of the NES memory.
    This is for done only testing the 6502 implementation and will be expanded
    afterwards.
*/
#[derive(Debug)]
pub struct Bus {
    pub data: [u8; 1 << 16], // TODO: make private when possible
}

impl Bus {
    pub(crate) fn new() -> Self {
        Bus { data: [0; 1 << 16] }
    }

    pub(crate) fn read_u8(&self, address: u16) -> u8 {
        self.data[address as usize]
    }
}
