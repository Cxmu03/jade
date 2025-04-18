use jade::cpu::instruction::InstructionCycle::Rts3;
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
    let md5 = Md5::new();

    let executable = md5.get_executable();
    let start_address = md5.get_start_address();

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

        //println!("{jade:?}");

        if perfect_snapshot != jade_snapshot {
            /*println!("{jade:?}");
            println!("{perfect_snapshot:?}");
            println!("{jade_snapshot:?}\n");*/
            errors += 1;
        }

        if jade.cpu.fetch == Some("BRK impl".to_owned()) {
            break;
        }

        if i % 100000 == 0 {
            println!("{i}");
        }

        i += 1;
    }

    println!("{:?}", perfect6502.create_status_snapshot());
    println!("{:?}", jade.create_status_snapshot());

    println!(
        "Detected {errors} errors in {} cycles ({}%)",
        jade.cpu.cycles,
        (f64::from(errors) / (jade.cpu.cycles as f64)) * 100f64
    );
}
