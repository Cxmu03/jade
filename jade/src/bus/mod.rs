pub trait Bus {
    fn new() -> Self;

    fn read_u8(&self, address: u16) -> u8;

    fn write_u8(&mut self, address: u16, value: u8);
}

const RAM_SIZE: u16 = 1 << 11; // 2kiB
const RAM_END: u16 = 4 * RAM_SIZE;

#[derive(Debug)]
pub struct NesBus {
    ram: [u8; RAM_SIZE as usize],
}

impl Bus for NesBus {
    fn new() -> Self {
        NesBus {
            ram: [0; RAM_SIZE as usize],
        }
    }

    fn read_u8(&self, address: u16) -> u8 {
        match address {
            0..RAM_END => self.ram[(address % RAM_SIZE) as usize],
            _ => todo!(),
        }
    }

    fn write_u8(&mut self, address: u16, value: u8) {
        match address {
            0..RAM_END => {
                self.ram[(address % RAM_SIZE) as usize] = value;
            }
            _ => todo!(),
        }
    }
}

pub struct TestBus {
    pub data: [u8; 1 << 16],
}

impl Bus for TestBus {
    fn new() -> Self {
        TestBus { data: [0; 1 << 16] }
    }

    fn read_u8(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    fn write_u8(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }
}
