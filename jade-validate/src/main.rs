use clap::Parser;
use jade_programs::{GenericJadeProgram, JadeProgram, Md5};
use jade_validate::cli::*;
use jade_validate::common::traits::*;
use jade_validate::common::types::*;
use jade_validate::emulators::perfect6502::bindings::*;
use jade_validate::emulators::{jade::Jade, perfect6502::Perfect6502};
use jade_validate::validate;
use std::fs::{File, OpenOptions};
use std::str::FromStr;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Validate {
            validator,
            generator,
            cycles,
            executable_command,
        } => {
            let mut validator: Box<dyn Validator> = validator.new_validator() as Box<dyn Validator>;
            let mut generator: Box<dyn Generator> = generator.new_generator() as Box<dyn Generator>;
            let program: Box<dyn JadeProgram> = match &executable_command {
                ExecutableCommand::WithBuiltin { name } => {
                    <Box<dyn JadeProgram>>::from_str(name).unwrap()
                }
                ExecutableCommand::WithFile { name, start_addr } => {
                    let mut file: File = OpenOptions::new()
                        .read(true)
                        .create(false)
                        .open(name)
                        .unwrap();
                    let length: u64 = file.metadata().unwrap().len();

                    let program: Box<dyn JadeProgram> = Box::new(GenericJadeProgram {
                        executable: vec![0u8; length as usize].into_boxed_slice(),
                        start_addr: *start_addr,
                        name: name.to_str().unwrap().to_owned(),
                    });

                    program
                }
            };

            let error_map = validate(&mut generator, &mut validator, &program, *cycles);
            println!(
                "Validated {} with {}:",
                generator.get_name(),
                validator.get_name()
            );
            println!("Ran program {} for {} cycles", program.get_name(), *cycles);
            println!(
                "Status errors: {}",
                error_map.get_count_of(ValidationError::Status)
            );
            println!("IO errors: {}", error_map.get_count_of(ValidationError::Io));
            println!(
                "Register errors: {}",
                error_map.get_count_of(ValidationError::Register)
            );
            println!(
                "Control Flow errors: {}",
                error_map.get_count_of(ValidationError::ControlFlow)
            );
        }
    }
    //let generator = cli.generator.new_generator();
    //let validator = cli.validator.new_validator();
    //let program: Box<dyn JadeProgram>;
}
