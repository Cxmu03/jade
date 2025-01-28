use jade::cpu::Cpu;
use std::env;
use std::time::Instant;

fn main() {
    let mut cpu: Cpu = Cpu::new();
    cpu.a = 0xaa;

    /*let program: &[u8; 24] = &[
        0xa9, 0x00, 0x20, 0x10, 0x00, 0x4c, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x40, 0xe8, 0x88, 0xe6, 0x0f, 0x38, 0x69, 0x02, 0x60,
    ];*/

    let padding = 0x0;

    /*  let program = &[0x18, 0xa9, 0x6a, 0xa2, 0x0a, 0x7D, 0xFE, 0x01, 0xa9];
    cpu.bus.data[0x01FE + 0xa] = 0xab; */

    //let program = &[0xa9, 0x12, 0x6d, 0x00, 0x01, 0xa9];
    //cpu.bus.data[0x0100] = 0x02;

    let program = &[0xa9, 0xa0, 0x65, 0x50, 0xa9];
    cpu.bus.data[0x0050] = 0x01;
    //let program = &[0xa9, 0x10, 0x08, 0xa9, 0xff, 0x28];

    /*let program = &[0xa9, 0x10, 0xa2, 0x05, 0x61, 0x50, 0xa2, 0x00];
    cpu.bus.data[0x0055] = 0x02;
    cpu.bus.data[0x0056] = 0x01;
    cpu.bus.data[0x0102] = 0x03;*/

    /*let program = &[0xa9, 0x10, 0xa0, 0xFF, 0x71, 0x50, 0xa2, 0x00];
    cpu.bus.data[0x0050] = 0x02;
    cpu.bus.data[0x0051] = 0x01;
    cpu.bus.data[0x0201] = 0x03;*/

    cpu.bus.data[padding..(program.len() + padding)].copy_from_slice(program);
    cpu.next_pc = padding as u16;

    let log: bool = env::var("LOG")
        .unwrap_or(String::from("false"))
        .parse()
        .expect("Environment variable to be a valid boolean");
    let iterations: usize = env::var("ITERATIONS")
        .unwrap_or(String::from("20"))
        .parse()
        .expect("Environment variable to be a valid unsigned integer");

    let start = Instant::now();

    for _ in 0..iterations {
        cpu.step_cycle();
        if log {
            println!(
                    "cycle: {:2}, a: {:02x} x: {:02x}, y: {:02x}, ab: {:04x}, db: {:02x}, r: {:?}, pc: {:04x}, sp: {:02x}, {:?}, {:?}, {}, p: {}",
                    cpu.cycles - 1, cpu.a, cpu.x, cpu.y, cpu.ab, cpu.db, cpu.r, cpu.pc, cpu.sp, cpu.fetch, cpu.execute, cpu.execution_state, cpu.p
                );
        }
    }

    let elapsed = start.elapsed().as_secs_f64();
    println!("Ran {iterations} cycles in {elapsed} seconds");
    let freq_mhz: f64 = ((iterations as f64) / elapsed) / 1000000f64;
    println!("Running at {freq_mhz} mHz");
}
