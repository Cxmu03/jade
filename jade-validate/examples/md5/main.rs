use jade_validate::{
    common::traits::*,
    emulators::{jade::Jade, perfect6502::Perfect6502},
};
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, SeekFrom};

// TODO: Move to main crate

fn main() {
    let start_addr = 0x0200;
    let mut perfect6502 = Perfect6502::new();
    let mut jade = Jade::new();

    let mut executable = OpenOptions::new()
        .read(true)
        .open("jade-validate/examples/md5/main")
        .unwrap();

    perfect6502
        .load_executable_from_file(&mut executable, start_addr)
        .unwrap();
    executable.seek(SeekFrom::Start(0)).unwrap();
    jade.load_executable_from_file(&mut executable, start_addr)
        .unwrap();

    perfect6502.set_reset_vector(start_addr);
    jade.set_reset_vector(start_addr);

    let (snapshot, new_pc) = perfect6502.reset().unwrap();
    jade.init_with_cpu_status(&snapshot, new_pc);

    let mut errors = 0;
    let mut i = 0;
    loop {
        let perfect_snapshot = perfect6502.step_cycle().unwrap();
        let jade_snapshot = jade.step_cycle().unwrap();

        println!("{jade:?}");

        if perfect_snapshot != jade_snapshot {
            println!("{perfect_snapshot:?}");
            println!("{jade_snapshot:?}");
            errors += 1;
        }

        if perfect_snapshot.a == 0xBE || perfect_snapshot.a == 0xEB {
            let mut dump_file = OpenOptions::new()
                .write(true)
                .create(true)
                .open("jade-validate/examples/md5/fin_dump.bin")
                .unwrap();

            perfect6502.dump_memory(&mut dump_file);

            break;
        }

        i += 1;
    }

    println!(
        "Detected {errors} errors in {} cycles ({}%)",
        jade.cpu.cycles,
        (f64::from(errors) / (jade.cpu.cycles as f64)) * 100f64
    );
}
