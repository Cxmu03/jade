pub mod common;
pub mod emulators;

use common::traits::*;
use emulators::perfect6502::Perfect6502;

fn main() {
    let mut chip = Perfect6502::new();
    println!("{:04x?}", chip.create_status_snapshot());
    for _ in 0..8 {
        chip.step_cycle();
    }
    for i in 0..10 {
        chip.step_cycle();
        println!("{i}: {:04x?}", chip.create_status_snapshot());
    }
    println!("Hello from the jade validation crate!");
}
