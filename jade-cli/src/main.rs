use jade::cpu::instruction_table::INSTRUCTIONS;
use jade::cpu::Cpu;

fn main() {
    let mut cpu: Cpu = Cpu::new();

    cpu.current_instr = 0x20;
    cpu.pc = 3;
    cpu.bus.data[2] = 0x20; // JSR abs, but isn't actually used for now
    cpu.bus.data[3] = 0x01;
    cpu.bus.data[4] = 0x10;

    for _ in 0..6 {
        let cycle = INSTRUCTIONS[cpu.current_instr].cycles[cpu.current_instr_step];
        cpu.execute_microcode_step();
        println!(
            "ab: {:4x}, db: {:2x}, r: {}, pc: {:4x}, sp: {:2x}, {cycle}",
            cpu.ab, cpu.db, cpu.r, cpu.pc, cpu.sp
        );
    }
}
