use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

use crate::Part;

pub fn solve(input_file_path: &Path, part: Part) -> () {
    match part {
        Part::Part1 => solve_part_1(input_file_path),
        Part::Part2 => panic!("Solution to part 2 is not implemented yet."),
    };
}

enum RPS {
    Rock,
    Paper,
    Scissors,
}

fn solve_part_1(input_file_path: &Path) -> i32 {
    let rps_encoding = HashMap::from([
        ("A", RPS::Rock),
        ("B", RPS::Paper),
        ("C", RPS::Scissors),
        ("X", RPS::Rock),
        ("Y", RPS::Paper),
        ("Z", RPS::Scissors),
    ]);

    // Converts e.g. "A Y\nB X" -> [["A", "Y"], ["B", "X"]]
    let games: Vec<Vec<String>> = read_to_string(input_file_path)
        .unwrap()
        .lines()
        // Split each line into substrings (&str)
        // then convert them into owned strings.
        .map(|line| line.split_whitespace().map(|s| s.to_string()).collect())
        .collect();

    let mut total_points: i32 = 0;

    for game in games {
        let opponent_choice = rps_encoding.get(&game[0].as_str()).unwrap();
        let my_choice = rps_encoding.get(&game[1].as_str()).unwrap();

        let choice_points: i32 = match my_choice {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        };

        let outcome_points: i32 = match opponent_choice {
            RPS::Rock => match my_choice {
                RPS::Rock => 3,
                RPS::Paper => 6,
                RPS::Scissors => 0,
            },
            RPS::Paper => match my_choice {
                RPS::Rock => 0,
                RPS::Paper => 3,
                RPS::Scissors => 6,
            },
            RPS::Scissors => match my_choice {
                RPS::Rock => 6,
                RPS::Paper => 0,
                RPS::Scissors => 3,
            },
        };

        total_points += choice_points;
        total_points += outcome_points;
    }

    println!("{}", total_points);
    total_points
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests;

    #[test]
    fn test_solve_part_1() {
        let input_file_path = tests::data_path("day_02_part_1.txt");
        assert_eq!(solve_part_1(input_file_path.as_path()), 15);
    }
}
