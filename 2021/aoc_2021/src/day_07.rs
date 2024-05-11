use std::fs::read_to_string;
use std::path::PathBuf;

use crate::{Config, Part};

pub fn solve(config: Config) {
    match config.part {
        Part::One => solve_part_1(config.input_path),
        Part::Two => solve_part_2(config.input_path),
    };
}

fn parse_input(input_path: PathBuf) -> Vec<i32> {
    let binding = read_to_string(input_path).unwrap();
    binding
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn solve_part_1(input_path: PathBuf) -> i32 {
    let mut input = parse_input(input_path);
    input.sort();

    let mid = input.len() / 2;
    let median = input[mid];

    let optimal_abs_diff = sum_abs_difference(&input, median);

    println!("{}", optimal_abs_diff);
    optimal_abs_diff
}

fn sum_abs_difference(positions: &Vec<i32>, target_position: i32) -> i32 {
    let mut sum_abs_difference: i32 = 0;
    for pos in positions {
        sum_abs_difference += (target_position - pos).abs()
    }
    sum_abs_difference
}

fn solve_part_2(input_path: PathBuf) -> i32 {
    let input = parse_input(input_path);
    panic!("No solution for part 2 yet.")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests;

    #[test]
    fn test_solve_part_1() {
        let input_path = tests::data_path("day_07_part_1.txt");
        assert_eq!(solve_part_1(input_path), 37);
    }

    #[test]
    fn test_solve_part_2() {
        let input_path = tests::data_path("day_01_part_1.txt");
        assert_eq!(solve_part_2(input_path), 0);
    }
}
