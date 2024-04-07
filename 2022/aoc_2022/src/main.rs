mod common;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;

use std::env;
use std::path::Path;

enum Part {
    Part1,
    Part2,
}

fn main() -> () {
    // Parse args into vector.
    let args: Vec<String> = env::args().collect();

    let day: &String = &args[1];
    let part: &String = &args[2];
    let input_file_path = Path::new(&args[3]);

    // Coerce part into an enum.
    let part: Part = match part.as_str() {
        "1" => Part::Part1,
        "2" => Part::Part2,
        _ => panic!("part must be 1 or 2, not {}", part),
    };

    match day.as_str() {
        "day_01" => day_01::solve(input_file_path, part),
        "day_02" => day_02::solve(input_file_path, part),
        "day_03" => day_03::solve(input_file_path, part),
        "day_04" => day_04::solve(input_file_path, part),
        "day_05" => day_05::solve(input_file_path, part),
        "day_06" => day_06::solve(input_file_path, part),
        "day_07" => day_07::solve(input_file_path, part),
        "day_08" => day_08::solve(input_file_path, part),
        other => panic!("No such day as {}", other),
    }
}
