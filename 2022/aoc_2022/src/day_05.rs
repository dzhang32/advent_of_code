use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

use crate::Part;

pub fn solve(input_file_path: &Path, part: Part) -> () {
    match part {
        Part::Part1 => solve_part_1(input_file_path),
        Part::Part2 => solve_part_2(input_file_path),
    };
}

fn solve_part_1(input_file_path: &Path) -> String {
    let binding = read_to_string(input_file_path).unwrap();
    let input = binding
        .lines()
        .group_by(|&x| x.is_empty())
        .into_iter()
        .map(|(_, group)| group.collect())
        .collect::<Vec<Vec<&str>>>();

    // Create a vector which will store N vectors.
    // Each inner vector represents each of the columns in our puzzle input.
    let puzzle = &input[0][..input[0].len() - 1];
    let columns = input[0][input[0].len() - 1];
    let mut stacks = parse_stacks(puzzle, columns);

    // Simulate each move of the puzzle.
    let instructions = &input[2];
    let moves = parse_instructions(instructions);

    for m in moves {
        let n = m[0];
        let from = m[1] as usize - 1;
        let to = m[2] as usize - 1;

        for _ in 0..n {
            let to_move = stacks[from].pop().unwrap();
            stacks[to].push(to_move);
        }
    }

    // Extract the top element of each stack.
    let mut top = String::new();
    for s in stacks {
        top.push(*s.last().unwrap());
    }

    println!("{}", top);
    top
}

fn solve_part_2(input_file_path: &Path) -> String {
    let binding = read_to_string(input_file_path).unwrap();
    let input = binding
        .lines()
        .group_by(|&x| x.is_empty())
        .into_iter()
        .map(|(_, group)| group.collect())
        .collect::<Vec<Vec<&str>>>();

    // Create a vector which will store N vectors.
    // Each inner vector represents each of the columns in our puzzle input.
    let puzzle = &input[0][..input[0].len() - 1];
    let columns = input[0][input[0].len() - 1];
    let mut stacks = parse_stacks(puzzle, columns);

    // Simulate each move of the puzzle.
    let instructions = &input[2];
    let moves = parse_instructions(instructions);

    for m in moves {
        let n = m[0] as usize;
        let from = m[1] as usize - 1;
        let to = m[2] as usize - 1;

        let mut to_reverse = Vec::with_capacity(n);

        for _ in 0..n {
            to_reverse.push(stacks[from].pop().unwrap());
        }

        for _ in 0..n {
            stacks[to].push(to_reverse.pop().unwrap());
        }
    }

    // Extract the top element of each stack.
    let mut top = String::new();
    for s in stacks {
        top.push(*s.last().unwrap());
    }

    println!("{}", top);
    top
}

fn parse_stacks(puzzle: &[&str], columns: &str) -> Vec<Vec<char>> {
    // Find the index in the string where each column resides.
    let mut column_indices: HashMap<usize, usize> = HashMap::new();
    for (i, c) in columns.chars().enumerate() {
        if !c.is_whitespace() {
            // Convert char to usize,
            // then -1 as Rust indexes from 0.
            let c_usize = c.to_digit(10).unwrap() as usize - 1;
            column_indices.insert(i, c_usize);
        }
    }

    // Create empty vector of N stacks, where N is the N column indices.
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(column_indices.len());
    for _ in 0..column_indices.len() {
        stacks.push(Vec::new());
    }

    // Populate each stack. rev() to ensure first element of stack represents
    // the bottom row of the puzzle.
    for &row in puzzle.iter().rev() {
        for (i, c) in row.chars().enumerate() {
            if column_indices.contains_key(&i) {
                if !c.is_whitespace() {
                    let column = column_indices.get(&i).unwrap();
                    stacks[*column].push(c);
                }
            }
        }
    }

    stacks
}

fn parse_instructions(instructions: &Vec<&str>) -> Vec<Vec<u32>> {
    let moves = instructions
        .iter()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    moves
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests;

    #[test]
    fn test_solve_part_1() {
        let input_file_path = tests::data_path("day_05_part_1.txt");
        assert_eq!(solve_part_1(input_file_path.as_path()), "CMZ");
    }

    #[test]
    fn test_solve_part_2() {
        let input_file_path = tests::data_path("day_05_part_1.txt");
        assert_eq!(solve_part_2(input_file_path.as_path()), "MCD");
    }
}
