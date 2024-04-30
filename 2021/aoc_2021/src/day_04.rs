use itertools::Itertools;
use std::cmp::max;
use std::path::PathBuf;
use std::{collections::HashMap, fs::read_to_string};

use crate::{Config, Part};

#[derive(Debug)]
struct Board {
    board: Vec<Vec<u32>>,
    round: Vec<Vec<usize>>,
    wins: usize,
}

impl Board {
    fn from(board: Vec<Vec<u32>>, number_round: &HashMap<u32, usize>) -> Board {
        // Store the round for which each number is called.
        // If the number is not called in any round,
        // set the round to an arbitrarily large value.
        let round = board
            .iter()
            .map(|r| {
                r.iter()
                    .map(|c| *number_round.get(c).unwrap_or(&usize::MAX))
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

        // Find the round when the board would win.
        // This is the min round for any row or col.
        let mut wins_rows_cols = round
            .iter()
            .map(|r| *r.iter().max().unwrap())
            .collect::<Vec<usize>>();

        for col_index in 0..round[0].len() {
            let mut max_col = usize::MIN;
            for row in &round {
                max_col = max(max_col, row[col_index]);
            }
            wins_rows_cols.push(max_col);
        }

        let wins = wins_rows_cols.iter().min().unwrap();

        Board {
            board,
            round,
            wins: *wins,
        }
    }

    fn score(&self) -> u32 {
        let mut sum_unmarked = 0;
        let mut winning_number = u32::MAX;
        for r in 0..self.round.len() {
            for c in 0..self.round[0].len() {
                if self.round[r][c] > self.wins {
                    sum_unmarked += self.board[r][c];
                }
                if self.round[r][c] == self.wins {
                    winning_number = self.board[r][c];
                }
            }
        }
        sum_unmarked * winning_number
    }
}

pub fn solve(config: Config) -> () {
    match config.part {
        Part::One => solve_part_1(config.input_path),
        Part::Two => solve_part_2(config.input_path),
    };
}

fn parse_input(input_path: PathBuf) -> (Vec<u32>, Vec<Vec<Vec<u32>>>) {
    let binding = read_to_string(input_path).unwrap();
    let input = binding
        .lines()
        .group_by(|x| x.is_empty())
        .into_iter()
        .map(|(_, g)| g.collect::<Vec<&str>>())
        .filter(|g| !g[0].is_empty())
        .collect::<Vec<Vec<&str>>>();

    let numbers = input[0][0]
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let boards = input[1..input.len()]
        .iter()
        .map(|l| {
            l.iter()
                .map(|&x| {
                    x.trim()
                        .split_whitespace()
                        .map(|y| y.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>()
        })
        .collect::<Vec<Vec<Vec<u32>>>>();

    (numbers, boards)
}

fn solve_part_1(input_path: PathBuf) -> u32 {
    let (numbers, boards_raw) = parse_input(input_path);

    let mut number_round: HashMap<u32, usize> = HashMap::new();
    for i in 0..numbers.len() {
        if !number_round.contains_key(&numbers[i]) {
            number_round.insert(numbers[i], i);
        }
    }

    let mut boards = Vec::with_capacity(boards_raw.len());
    for board in boards_raw {
        boards.push(Board::from(board, &number_round));
    }

    let mut winning_board = &boards[0];
    let mut min_wins = usize::MAX;
    for board in boards.iter() {
        if board.wins < min_wins {
            min_wins = board.wins;
            winning_board = board;
        }
    }

    let winning_score = winning_board.score();

    println!("{:?}", winning_score);
    winning_score
}

fn solve_part_2(input_path: PathBuf) -> u32 {
    let (numbers, boards_raw) = parse_input(input_path);

    let mut number_round: HashMap<u32, usize> = HashMap::new();
    for i in 0..numbers.len() {
        if !number_round.contains_key(&numbers[i]) {
            number_round.insert(numbers[i], i);
        }
    }

    let mut boards = Vec::with_capacity(boards_raw.len());
    for board in boards_raw {
        boards.push(Board::from(board, &number_round));
    }

    let mut losing_board = &boards[0];
    let mut max_wins = usize::MIN;
    for board in boards.iter() {
        if board.wins > max_wins {
            max_wins = board.wins;
            losing_board = board;
        }
    }

    let losing_score = losing_board.score();

    println!("{:?}", losing_score);
    losing_score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests;

    #[test]
    fn test_solve_part_1() {
        let input_path = tests::data_path("day_04_part_1.txt");
        assert_eq!(solve_part_1(input_path), 4512);
    }

    #[test]
    fn test_solve_part_2() {
        let input_path = tests::data_path("day_04_part_1.txt");
        assert_eq!(solve_part_2(input_path), 1924);
    }
}
