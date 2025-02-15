mod instructions;

#[macro_export]
macro_rules! test_init_cpu {
    ($program: expr) => {{
        use crate::bus::{Bus, TestBus};

        let mut bus = TestBus::new();
        let mut cpu = Cpu::new();

        bus.data[0..$program.len()].copy_from_slice($program);
        cpu.step_cycle(&mut bus); // Get fetch out the way

        (cpu, bus)
    }};
    ($program: expr, $padding: expr) => {{
        use crate::bus::{Bus, TestBus};

        let mut bus = TestBus::new();
        let mut cpu = Cpu::new();

        bus.data[$padding..($program.len() + $padding)].copy_from_slice($program);
        cpu.next_pc = $padding;
        cpu.step_cycle(&mut bus);

        (cpu, bus)
    }};
}
