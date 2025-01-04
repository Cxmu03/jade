use jade::cpu::Cpu;
use std::env;
use std::time::Instant;

fn main() {
    let mut cpu: Cpu = Cpu::new();
    cpu.a = 0xaa;

    let program: &[u8; 24] = &[
        0xa9, 0x00, 0x20, 0x10, 0x00, 0x4c, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x40, 0xe8, 0x88, 0xe6, 0x0f, 0x38, 0x69, 0x02, 0x60,
    ];

    cpu.bus.data[0..program.len()].copy_from_slice(program);
    let mut counter = 0;

    let log: bool = env::var("LOG")
        .unwrap_or(String::from("false"))
        .parse()
        .expect("Environment variable to be a valid boolean");
    let iterations: usize = env::var("ITERATIONS")
        .unwrap_or(String::from("20"))
        .parse()
        .expect("Environment variable to be a valid unsigned integer");

    let start = Instant::now();

    loop {
        cpu.step_cycle();
        if log {
            println!(
                "cycle: {:2}, a: {:02x} x: {:02x}, y: {:02x}, ab: {:04x}, db: {:02x}, r: {}, pc: {:04x}, sp: {:02x}, {:?}, {:?}, {}, p: {}",
                counter, cpu.a, cpu.x, cpu.y, cpu.ab, cpu.db, cpu.r, cpu.pc, cpu.sp, cpu.fetch, cpu.execute, cpu.execution_state, cpu.p
            );
        }
        counter += 1;

        if counter == iterations {
            break;
        }
    }

    let elapsed = start.elapsed().as_secs_f64();
    println!("Ran {iterations} cycles in {elapsed} seconds");
    let freq_mhz: f64 = ((iterations as f64) / elapsed) / 1000000f64;
    println!("Running at {freq_mhz} mHz");
}
