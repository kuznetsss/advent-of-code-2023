mod day_1;
mod error;

use std::process::ExitCode;

use clap::{Parser, Subcommand};
use error::AocError;

#[derive(Debug, Subcommand, Clone)]
enum Command {
    /// run day_1 solution
    DAY1,
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,

    #[arg(short, long, default_value_t = String::from("input.txt"))]
    input_file: String,
}

static DEFAULT_COMMAND: Command = Command::DAY1;

fn main_impl() -> Result<(), AocError> {
    let args = Args::parse();
    match args.command.unwrap_or(DEFAULT_COMMAND.clone()) {
        Command::DAY1 => {
            let result = day_1::day_1(&args.input_file)?;
            println!("The result is {result}");
        }
    };
    Ok(())
}

fn main() -> ExitCode {
    match main_impl() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
