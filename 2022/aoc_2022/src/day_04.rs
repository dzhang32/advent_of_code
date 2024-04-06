use std::fs::read_to_string;
use std::path::Path;

use crate::Part;

pub fn solve(input_file_path: &Path, part: Part) -> () {
    match part {
        Part::Part1 => solve_part(input_file_path, Part::Part1),
        Part::Part2 => solve_part(input_file_path, Part::Part2),
    };
}

fn solve_part(input_file_path: &Path, part: Part) -> u32 {
    let binding = read_to_string(input_file_path).unwrap();
    let input = binding
        .lines()
        .map(|line| {
            line.split([',', '-'])
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut n_contained: u32 = 0;
    for pair in &input {
        let pair_1 = [pair[0], pair[1]];
        let pair_2 = [pair[2], pair[3]];

        match part {
            Part::Part1 => {
                // If pair 1 completely contains pair 2.
                if pair_1[0] <= pair_2[0] && pair_1[1] >= pair_2[1] {
                    n_contained += 1;
                // Or, if pair 2 completely contains pair 1.
                } else if pair[0] >= pair_2[0] && pair_1[1] <= pair_2[1] {
                    n_contained += 1;
                }
            }
            Part::Part2 => {
                if pair_1[1] >= pair_2[0] && pair_1[0] <= pair_2[1] {
                    n_contained += 1;
                }
            }
        }
    }

    println!("{}", n_contained);
    n_contained
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests;

    #[test]
    fn test_solve_part_1() {
        let input_file_path = tests::data_path("day_04_part_1.txt");
        assert_eq!(solve_part(input_file_path.as_path(), Part::Part1), 2);
    }

    #[test]
    fn test_solve_part_2() {
        let input_file_path = tests::data_path("day_04_part_1.txt");
        assert_eq!(solve_part(input_file_path.as_path(), Part::Part2), 4);
    }
}
