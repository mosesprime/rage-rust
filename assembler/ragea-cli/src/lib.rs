use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Target architecture. (ex. x86-64, ARM, ...)
    target_arch: String,

    /// Target operating system. (ex. windows, linux, ...)
    target_os: String,

    /// Turn on debugging information.
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// Set the input file path.
    #[arg(short, long)]
    input: Option<PathBuf>,

    /// Set the output file path.
    #[arg(short, long)]
    output: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {}

pub fn run() {
    let cli = Cli::parse();
    println!("{cli:?}");
}
