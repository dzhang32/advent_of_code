use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::Part;

pub fn solve(input_file_path: &Path, part: Part) -> () {
    match part {
        Part::Part1 => solve_part_1(input_file_path),
        Part::Part2 => panic!("No solution to part 2 yet."),
    };
}

fn solve_part_1(input_file_path: &Path) -> i32 {
    let file = File::open(input_file_path).expect("File not found.");
    let reader = BufReader::new(file);

    let mut cum_sum: i32 = 0;
    let mut max_sum: i32 = 0;

    for line in reader.lines() {
        let line_unwrapped = line.expect("Unable to read line.");
        if line_unwrapped.is_empty() {
            cum_sum = 0
        } else {
            let line_int: i32 = line_unwrapped.parse().expect("Cannot parse line to int.");
            cum_sum += line_int;
            max_sum = max(max_sum, cum_sum)
        }
    }

    println!("{}", max_sum);

    max_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn test_solve_part_1() {
        let input_file_path = common::tests::data_path("day_01_part_1.txt");
        assert_eq!(solve_part_1(input_file_path.as_path()), 24000);
    }
}
