use jade::cpu::instruction_table::INSTRUCTIONS;
use jade::cpu::Cpu;

fn main() {
    let mut cpu: Cpu = Cpu::new();
    cpu.a = 0xaa;

    let program: &[u8; 24] = &[
        0xa9, 0x00, 0x20, 0x10, 0x00, 0x4c, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x40, 0xe8, 0x88, 0xe6, 0x0f, 0x38, 0x69, 0x02, 0x60,
    ];

    cpu.bus.data[0..program.len()].copy_from_slice(program);
    let mut counter = 0;

    loop {
        cpu.step_cycle();
        println!(
            "cycle: {:2}, a: {:02x} x: {:02x}, ab: {:04x}, db: {:02x}, r: {}, pc: {:04x}, sp: {:02x}, {:?}, {:?}, {}",
            counter, cpu.a, cpu.x, cpu.ab, cpu.db, cpu.r, cpu.pc, cpu.sp, cpu.fetch, cpu.execute, cpu.execution_state
        );
        counter += 1;
    }
}
