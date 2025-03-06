use crate::{GeneratorType, ValidatorType};
use clap_derive::{Args, Parser, Subcommand};
use clap_num::maybe_hex;
use std::path::PathBuf;
use std::str::FromStr;
use thiserror::Error;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Validate {
        /// The validating emulator. Possible options are <Perfect6502>.
        #[arg(long, short)]
        validator: ValidatorType,
        /// The emulator to be validated. Possible options are <Jade>.
        #[arg(long, short)]
        generator: GeneratorType,
        /// The amount of cycles that the validation runs for.
        #[arg(long, short)]
        cycles: usize,
        #[command(subcommand)]
        executable_command: ExecutableCommand,
    },
}

#[derive(Subcommand)]
#[command(rename_all = "kebab-case")]
pub enum ExecutableCommand {
    /// Loads a builtin program
    #[command(alias = "p")]
    WithBuiltin {
        /// The builtin program to execute. Possible values are <md5>.
        name: String,
    },
    /// Loads a program from a file
    #[command(alias = "f")]
    WithFile {
        /// The name of the file which contains the executable.
        #[arg(long, short)]
        name: PathBuf,
        /// The address at which to load the program. Can be hex or decimal.
        #[arg(long, short, value_parser=maybe_hex::<u16>)]
        start_addr: u16,
    },
}
