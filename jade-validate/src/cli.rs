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
        #[arg(long, short)]
        validator: ValidatorType,
        #[arg(long, short)]
        generator: GeneratorType,
        #[arg(long, short)]
        cycles: usize,
        #[command(subcommand)]
        executable_command: ExecutableCommand,
    },
}

#[derive(Subcommand)]
#[command(rename_all = "kebab-case")]
pub enum ExecutableCommand {
    #[command(alias = "p")]
    WithBuiltin { name: String },
    #[command(alias = "f")]
    WithFile {
        #[arg(long, short)]
        name: PathBuf,
        #[arg(long, short, value_parser=maybe_hex::<u16>)]
        start_addr: u16,
    },
}
