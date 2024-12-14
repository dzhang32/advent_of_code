mod day_1;

use clap::Parser;
use std::process;
use std::error::Error;

/// Parse input arguments.
#[derive(Parser)]
#[command(version, about = "Advent of code 2024 solutions")]
struct Cli {
    /// The day.  
    #[arg(short, long, value_name = "1-25", required = true)]
    pub day: i32,

    /// The part.
    #[arg(short, long, value_name = "1/2", required = true)]
    pub part: i32,
}

fn main() {
    let cli = Cli::parse();

    match run(cli) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

fn run(cli: Cli) -> Result<(), String> {
    match cli.day {
        1 => day_1::solve(cli.part), 
        _ => return Err(format!("Day {} not implemented.", cli.day)),
    }
    Ok(())
}   