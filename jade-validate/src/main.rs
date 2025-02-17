mod common;
mod cpu_status;
mod emulators;

use emulators::perfect6502::bindings::initAndResetChip;

fn main() {
    unsafe {
        let chip = initAndResetChip();
        println!("{:?}", chip);
    }
    println!("Hello from the jade validation crate!");
}
