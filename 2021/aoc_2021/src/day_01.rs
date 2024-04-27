use std::fs::read_to_string;
use std::path::PathBuf;

use crate::{Config, Part};

pub fn solve(config: Config) -> () {
    match config.part {
        Part::One => solve_part_1(config.input_path),
        Part::Two => solve_part_2(config.input_path),
    };
}

fn parse_input(input_path: PathBuf) -> Vec<u32> {
    let binding = read_to_string(input_path).unwrap();
    let input = binding
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    input
}

fn solve_part_1(input_path: PathBuf) -> u32 {
    let input = parse_input(input_path);
    let mut prev_depth = input[0];
    let mut increased_count: u32 = 0;
    for i in 1..input.len() {
        if prev_depth < input[i] {
            increased_count += 1
        }
        prev_depth = input[i]
    }

    println!("{}", increased_count);
    increased_count
}

fn solve_part_2(input_path: PathBuf) -> u32 {
    let input = parse_input(input_path);
    let k = 3;
    let mut slow = 0;
    let mut fast = 0;
    let mut window_sum = 0;
    let mut increased_count = 0;

    for _ in 0..input.len() {
        if fast < k {
            window_sum += input[fast];
        } else {
            let mut next_window_sum = window_sum;
            next_window_sum -= input[slow];
            next_window_sum += input[fast];
            if window_sum < next_window_sum {
                increased_count += 1
            }

            window_sum = next_window_sum;
            slow += 1;
        }
        fast += 1;
    }

    println!("{}", increased_count);
    increased_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests;

    #[test]
    fn test_solve_part_1() {
        let input_path = tests::data_path("day_01_part_1.txt");
        assert_eq!(solve_part_1(input_path), 7);
    }

    #[test]
    fn test_solve_part_2() {
        let input_path = tests::data_path("day_01_part_1.txt");
        assert_eq!(solve_part_2(input_path), 5);
    }
}
