pub mod common;
pub mod emulators;

use common::traits::*;
use emulators::perfect6502::bindings::*;
use emulators::{jade::Jade, perfect6502::Perfect6502};

fn main() {
    let mut executable = [0; 1 << 16];

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
    executable[0x303] = 0x40;

    let mut perfect6502_emu = Perfect6502::new();
    let mut jade_emu = Jade::new();

    perfect6502_emu.load_executable_to(&executable, 0x00);
    jade_emu.load_executable_to(&executable, 0x00);

    let (initial_snapshot, new_pc) = perfect6502_emu.reset().unwrap();
    jade_emu.init_with_cpu_status(&initial_snapshot, new_pc);

    for i in 0..40 {
        let validator_status = perfect6502_emu.step_cycle().unwrap();
        let generator_status = jade_emu.step_cycle().unwrap();

        println!(
                "cycle: {:2}, a: {:02x} x: {:02x}, y: {:02x}, ab: {:04x}, db: {:02x}, r: {:?}, pc: {:04x}, sp: {:02x}, {:?}, {:?}, {}, p: {}, res: {}",
                jade_emu.cpu.cycles - 1, jade_emu.cpu.a, jade_emu.cpu.x, jade_emu.cpu.y, jade_emu.cpu.ab, jade_emu.cpu.db, jade_emu.cpu.r, jade_emu.cpu.pc, jade_emu.cpu.sp, jade_emu.cpu.fetch, jade_emu.cpu.execute, jade_emu.cpu.execution_state, jade_emu.cpu.p, jade_emu.cpu.reset
            );
        if generator_status != validator_status {
            println!(
                "States at index {i} don't match:\nvalidator: {:?}\ngenerator: {:?}",
                validator_status, generator_status
            )
        }
    }
}
