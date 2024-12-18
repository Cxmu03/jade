use jade::cpu::Cpu;

fn main() {
    let mut cpu: Cpu = Cpu::new();

    cpu.current_instr = 0xa9;
    cpu.pc = 1;
    cpu.bus.data[0] = 0xa9; // LDA imm, but isn't actually used for now
    cpu.bus.data[1] = 100;

    println!("a: {}, db: {}, ab: {}", cpu.a, cpu.db, cpu.ab);
    cpu.execute_microcode_step();
    println!("a: {}, db: {}, ab: {}", cpu.a, cpu.db, cpu.ab);
    cpu.execute_microcode_step();
    println!("a: {}, db: {}, ab: {}", cpu.a, cpu.db, cpu.ab);
}
