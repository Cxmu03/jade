use super::Bus;

mod instructions;

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

#[macro_export]
macro_rules! test_init_cpu {
    ($program: expr) => {{
        use crate::bus::Bus;
        use crate::cpu::tests::TestBus;

        let bus = TestBus::new();
        let mut cpu = Cpu::new(bus);

        cpu.bus.data[0..$program.len()].copy_from_slice($program);
        cpu.step_cycle(); // Get fetch out the way

        cpu
    }};
    ($program: expr, $padding: expr) => {{
        use crate::bus::Bus;
        use crate::cpu::tests::TestBus;

        let bus = TestBus::new();
        let mut cpu = Cpu::new(bus);

        cpu.bus.data[$padding..($program.len() + $padding)].copy_from_slice($program);
        cpu.next_pc = $padding;
        cpu.step_cycle();

        cpu
    }};
}
