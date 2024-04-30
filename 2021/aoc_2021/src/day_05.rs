use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;

use crate::{Config, Part};

#[derive(Debug)]
struct Coords {
    start: (i32, i32),
    end: (i32, i32),
}

impl Coords {
    fn from(line: &str) -> Coords {
        let coords = line
            .split(" -> ")
            .map(|x| {
                x.split(",")
                    .map(|y| y.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();
        Coords {
            start: (coords[0][0], coords[0][1]),
            end: (coords[1][0], coords[1][1]),
        }
    }

    fn store(&self, positions: &mut HashMap<(i32, i32), i32>, part: Part) -> () {
        let (x_start, y_start) = self.start;
        let (x_end, y_end) = self.end;

        if x_start != x_end && y_start != y_end {
            match part {
                Part::One => return (),
                Part::Two => self.store_diagonal(x_start, x_end, y_start, y_end, positions),
            }
        } else if x_start != x_end {
            self.store_horizontal_vertical("x", x_start, x_end, y_start, positions);
        } else {
            self.store_horizontal_vertical("y", y_start, y_end, x_start, positions);
        };
    }

    fn store_horizontal_vertical(
        &self,
        diff_x_y: &str,
        diff_start: i32,
        diff_end: i32,
        other_pos: i32,
        positions: &mut HashMap<(i32, i32), i32>,
    ) -> () {
        let (diff_start, diff_end) = if diff_end < diff_start {
            (diff_end, diff_start)
        } else {
            (diff_start, diff_end)
        };

        for diff_pos in diff_start..=diff_end {
            let to_store: (i32, i32) = match diff_x_y {
                "x" => (diff_pos, other_pos),
                "y" => (other_pos, diff_pos),
                _ => panic!("Unexpected diff value."),
            };
            let count = positions.entry(to_store).or_insert(0);
            *count += 1;
        }
    }

    fn store_diagonal(
        &self,
        x_start: i32,
        x_end: i32,
        y_start: i32,
        y_end: i32,
        positions: &mut HashMap<(i32, i32), i32>,
    ) -> () {
        if (y_start - y_end).abs() != (x_start - x_end).abs() {
            panic!("We only accept 45 degree diagonal lines.")
        }

        let diff = (x_start - x_end).abs();
        let x_pos_neg: &str = if x_end < x_start { "neg" } else { "pos" };
        let y_pos_neg: &str = if y_end < y_start { "neg" } else { "pos" };

        let mut x_start_to_increment: i32 = x_start;
        let mut y_start_to_increment: i32 = y_start;

        for _ in 0..=diff {
            let count = positions
                .entry((x_start_to_increment, y_start_to_increment))
                .or_insert(0);
            *count += 1;

            if x_pos_neg == "pos" {
                x_start_to_increment += 1
            } else {
                x_start_to_increment -= 1
            }

            if y_pos_neg == "pos" {
                y_start_to_increment += 1
            } else {
                y_start_to_increment -= 1
            }
        }
    }
}

pub fn solve(config: Config) -> () {
    match config.part {
        Part::One => solve_part_1(config.input_path),
        Part::Two => solve_part_2(config.input_path),
    };
}

fn parse_input(input_path: PathBuf) -> Vec<Coords> {
    let binding: String = read_to_string(input_path).unwrap();
    let input = binding
        .lines()
        .map(|l| Coords::from(l))
        .collect::<Vec<Coords>>();
    input
}

fn solve_part_1(input_path: PathBuf) -> i32 {
    let input = parse_input(input_path);
    let mut positions = HashMap::new();

    for coord in input {
        coord.store(&mut positions, Part::One);
    }

    let mut count_above_2 = 0;
    for (_, count) in &positions {
        if *count >= 2 {
            count_above_2 += 1
        }
    }
    println!("{:?}", count_above_2);
    count_above_2
}

fn solve_part_2(input_path: PathBuf) -> i32 {
    let input = parse_input(input_path);
    let mut positions = HashMap::new();

    for coord in input {
        coord.store(&mut positions, Part::Two);
    }

    let mut count_above_2 = 0;
    for (_, count) in &positions {
        if *count >= 2 {
            count_above_2 += 1
        }
    }
    println!("{:#?}", count_above_2);
    count_above_2
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests;

    #[test]
    fn test_solve_part_1() {
        let input_path = tests::data_path("day_05_part_1.txt");
        assert_eq!(solve_part_1(input_path), 5);
    }

    #[test]
    fn test_solve_part_2() {
        let input_path = tests::data_path("day_05_part_1.txt");
        assert_eq!(solve_part_2(input_path), 12);
    }
}
