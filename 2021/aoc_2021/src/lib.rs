use std::error::Error;
use std::path::PathBuf;

mod common;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;

pub enum Part {
    One,
    Two,
}

pub struct Config {
    pub day: u8,
    pub part: Part,
    pub input_path: PathBuf,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // First argument is the name of the binary.
        args.next();

        let day = match args.next() {
            Some(arg) => match arg.parse::<u8>() {
                Ok(arg) => arg,
                Err(_) => return Err("Day could not be parsed to int."),
            },
            None => return Err("Day not supplied."),
        };

        let part = match args.next() {
            Some(arg) => match arg.as_str() {
                "1" => Part::One,
                "2" => Part::Two,
                _ => return Err("Part must be one of '1' or '2'."),
            },
            None => return Err("Part not supplied."),
        };

        let input_path = match args.next() {
            Some(arg) => {
                let input_path = PathBuf::from(arg);
                if !input_path.is_file() {
                    return Err("Supplied path must point to an existing file.");
                }
                input_path
            }
            None => return Err("Path to input file not supplied."),
        };

        Ok(Config {
            day,
            part,
            input_path,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.day {
        1 => day_01::solve(config),
        2 => day_02::solve(config),
        3 => day_03::solve(config),
        4 => day_04::solve(config),
        5 => day_05::solve(config),
        6 => day_06::solve(config),
        7 => day_07::solve(config),
        other => return Err(format!("Solution for day {} not found.", other).into()),
    };

    Ok(())
}
