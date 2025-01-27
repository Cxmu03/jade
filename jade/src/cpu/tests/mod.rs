mod instructions;

#[macro_export]
macro_rules! test_init_cpu {
    ($program: expr) => {{
        let mut cpu = Cpu::new();

        cpu.bus.data[0..$program.len()].copy_from_slice($program);
        cpu.step_cycle(); // Get fetch out the way

        cpu
    }};
    ($program: expr, $padding: expr) => {{
        let mut cpu = Cpu::new();

        cpu.bus.data[$padding..($program.len() + $padding)].copy_from_slice($program);
        cpu.next_pc = $padding;
        cpu.step_cycle();

        cpu
    }};
}
