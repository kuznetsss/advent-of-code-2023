mod day_1;
mod day_2;

mod command;
mod error;
mod input_provider;

use command::{Command, DEFAULT_COMMAND};
use error::AocError;
use input_provider::InputProvider;

use clap::Parser;

use std::process::ExitCode;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

fn main_impl() -> Result<(), AocError> {
    let args = Args::parse();
    let result: u32;
    let command = args.command.unwrap_or(DEFAULT_COMMAND.clone());
    let input = InputProvider::new()?.get_input(command.clone().into())?;
    match command {
        Command::DAY1 => {
            result = day_1::day_1(&input)?;
        }
        Command::DAY2 => {
            result = day_2::day_2(&input)?;
        }
        Command::DAY2PART2 => {
            result = day_2::day_2_part_2(&input)?;
        }
    };
    println!("The result is {result}");
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
