use std::fs::read_to_string;
use std::path::PathBuf;

use crate::{Config, Part};

pub fn solve(config: Config) -> () {
    match config.part {
        Part::One => solve_part(config.input_path, 80),
        Part::Two => solve_part(config.input_path, 256),
    };
}

fn parse_input(input_path: PathBuf) -> Vec<usize> {
    let binding = read_to_string(input_path).unwrap();
    let mut input: Vec<Vec<usize>> = binding
        .lines()
        .map(|l| {
            l.split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|&x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    input.pop().unwrap()
}

fn solve_part(input_path: PathBuf, days: usize) -> u64 {
    let input = parse_input(input_path);
    const BIRTH_TIMER: usize = 8;
    const RESTART_TIMER: usize = 6;

    let mut fish_ages_count: [u64; 9] = [0; BIRTH_TIMER + 1];

    for age in input {
        fish_ages_count[age] += 1;
    }

    for _ in 0..days {
        let birth_count: u64 = fish_ages_count[0];

        for age in 1..fish_ages_count.len() {
            fish_ages_count[age - 1] = fish_ages_count[age];
        }

        fish_ages_count[BIRTH_TIMER] = birth_count;
        fish_ages_count[RESTART_TIMER] += birth_count;
    }

    let total_fish: u64 = fish_ages_count.iter().sum();
    println!("{:?}", total_fish);
    total_fish
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests;

    #[test]
    fn test_solve_part_1() {
        let input_path = tests::data_path("day_06_part_1.txt");
        assert_eq!(solve_part(input_path, 80), 5934);
    }

    #[test]
    fn test_solve_part_2() {
        let input_path = tests::data_path("day_06_part_1.txt");
        assert_eq!(solve_part(input_path, 256), 26984457539);
    }
}
