use clap::Parser;
use jade_programs::{JadeProgram, Md5};
use jade_validate::cli::Cli;
use jade_validate::common::traits::*;
use jade_validate::emulators::perfect6502::bindings::*;
use jade_validate::emulators::{jade::Jade, perfect6502::Perfect6502};

fn main() {
    let cli = Cli::parse();
    let a: Box<dyn JadeProgram> = Box::new(Md5 {});
    /*let mut executable = [0; 1 << 16];

    executable[0] = 0xa9;
    executable[1] = 0xff;
    executable[2] = 0xa2;
    executable[3] = 0xf3;
    executable[5..100].fill(0xea);
    executable[0x200] = 0x40;
    executable[0xfffa] = 0x00;
    executable[0xfffb] = 0x02;
    executable[0xfffc] = 0x00;
    executable[0xfffd] = 0x03;
    executable[0x300] = 0xa2;
    executable[0x301] = 0xff;
    executable[0x302] = 0x9a;
    executable[0x303] = 0x40;*/

    /*let executable: [u8; 24] = [
        0xa9, 0x00, 0x20, 0x10, 0x00, 0x4c, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x40, 0xe8, 0x88, 0xe6, 0x0f, 0x38, 0x69, 0x02, 0x60,
    ];*/

    /*let mut executable = [0xa9, 0x00, 0xc0, 0x6e, 0xa2, 0xff];

    let mut perfect6502_emu = Perfect6502::new();
    let mut jade_emu = Jade::new();

    perfect6502_emu.load_executable_to(&executable, 0x00);
    jade_emu.load_executable_to(&executable, 0x00);

    let (initial_snapshot, new_pc) = perfect6502_emu.reset().unwrap();
    jade_emu.init_with_cpu_status(&initial_snapshot, new_pc);

    let iterations = 100;
    let mut error_count = 0;

    for i in 0..iterations {
        let validator_status = perfect6502_emu.step_cycle().unwrap();
        let generator_status = jade_emu.step_cycle().unwrap();

        println!(
                "cycle: {:2}, a: {:02x} x: {:02x}, y: {:02x}, ab: {:04x}, db: {:02x}, r: {:?}, pc: {:04x}, sp: {:02x}, {:?}, {:?}, {}, p: {}, res: {}",
                jade_emu.cpu.cycles - 1, jade_emu.cpu.a, jade_emu.cpu.x, jade_emu.cpu.y, jade_emu.cpu.ab, jade_emu.cpu.db, jade_emu.cpu.r, jade_emu.cpu.pc, jade_emu.cpu.sp, jade_emu.cpu.fetch, jade_emu.cpu.execute, jade_emu.cpu.execution_state, jade_emu.cpu.p, jade_emu.cpu.reset
            );
        if generator_status != validator_status {
            error_count += 1;
            println!(
                "States at index {i} don't match:\nvalidator: {:?}\ngenerator: {:?}",
                validator_status, generator_status
            )
        }
    }

    let error_rate = f64::from(error_count) / f64::from(iterations);
    println!("Detected {error_count} errors in {iterations} iterations (error_rate={error_rate})");*/
}
