mod common;
mod day_01;

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
        other => panic!("No such day as {}", other),
    }
}
