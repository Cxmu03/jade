use std::path::PathBuf;
use std::str::FromStr;

use clap_derive::{Args, Parser, Subcommand};
use clap_num::maybe_hex;
use thiserror::Error;

use crate::{GeneratorType, ValidatorType};

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
    Run {
        #[arg(long, short)]
        emulator: ValidatorType,
        #[arg(long, short)]
        cycles: usize,
        #[arg(long, short, default_value_t = true)]
        log: bool,
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
        #[command(subcommand)]
        exit_condition: Option<ExitConditionCommand>,
    },
    /// Loads a program from a file
    #[command(alias = "f")]
    WithFile {
        /// The name of the file which contains the executable.
        #[arg(long, short)]
        name: PathBuf,
        /// The address at which to start the program. Can be hex or decimal.
        #[arg(long, short, value_parser=maybe_hex::<u16>)]
        start_addr: u16,
        /// The address at which to load the program. Can be hex or decimal.
        /// Optional, default is start_addr.
        #[arg(long, short, value_parser=maybe_hex::<u16>)]
        load_addr: Option<u16>,
        #[command(subcommand)]
        exit_condition: Option<ExitConditionCommand>,
    },
}

#[derive(Subcommand, Clone)]
#[command(rename_all = "kebab-case")]
pub enum ExitConditionCommand {
    /// The program will terminate upon detecting a repeating program counter
    #[clap(alias = "exit-trap")]
    ExitOnTrap,
    /// The program counter at which the program will terminate. May be decimal or hex
    #[clap(alias = "exit-pc-eq")]
    ExitOnProgramCounterEquals {
        #[arg(value_parser=maybe_hex::<u16>)]
        pc: u16,
    },
    /// The minimum program counter below which the program will terminate. May be decimal or hex
    #[clap(alias = "exit-pc-lt")]
    ExitOnProgramCounterLessThan {
        #[arg(value_parser=maybe_hex::<u16>)]
        min_pc: u16,
    },
    /// The maximum program counter above which the program will terminate. May be decimal or hex
    #[clap(alias = "exit-pc-gt")]
    ExitOnProgramCounterGreaterThan {
        #[arg(value_parser=maybe_hex::<u16>)]
        max_pc: u16,
    },
}

impl std::fmt::Display for ExitConditionCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ExitConditionCommand::*;

        match self {
            ExitOnTrap => write!(f, "trap detected"),
            ExitOnProgramCounterEquals { pc } => write!(f, "pc == 0x{:04x}", pc),
            ExitOnProgramCounterGreaterThan { max_pc } => write!(f, "pc > 0x{:04x}", max_pc),
            ExitOnProgramCounterLessThan { min_pc } => write!(f, "pc < 0x{:04x}", min_pc),
        }
    }
}
