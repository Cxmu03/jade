use jade_programs::*;
use jade_validate::{
    common::traits::*,
    emulators::{jade::Jade, perfect6502::Perfect6502},
};
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, SeekFrom};

fn main() {
    let mut perfect6502 = Perfect6502::new();
    let mut jade = Jade::new();

    let executable = Md5::get_executable();
    let start_address = Md5::get_start_address();

    perfect6502
        .load_executable_to(&executable, start_address)
        .unwrap();
    jade.load_executable_to(&executable, start_address).unwrap();

    perfect6502.set_reset_vector(start_address);
    jade.set_reset_vector(start_address);

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
