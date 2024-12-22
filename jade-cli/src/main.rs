use jade::cpu::instruction_table::INSTRUCTIONS;
use jade::cpu::Cpu;

fn main() {
    let mut cpu: Cpu = Cpu::new();

    cpu.current_instr = 0x20;
    cpu.pc = 2;
    cpu.next_pc = 2; // TODO: should not be needed to set next_pc manually, maybe have some init method
    cpu.bus.data[2] = 0x20;
    cpu.bus.data[3] = 0x01;
    cpu.bus.data[4] = 0x10;

    for _ in 0..7 {
        let cycle = INSTRUCTIONS[cpu.current_instr].cycles[cpu.current_instr_step];
        cpu.step_cycle();
        println!(
            "ab: {:04x}, db: {:02x}, r: {}, pc: {:04x}, sp: {:02x}, {cycle}, {}",
            cpu.ab, cpu.db, cpu.r, cpu.pc, cpu.sp, cpu.execution_state
        );
    }
}
