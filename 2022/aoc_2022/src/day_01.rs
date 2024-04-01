use std::cmp::{max, Reverse};
use std::collections::BinaryHeap;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::Part;

pub fn solve(input_file_path: &Path, part: Part) -> () {
    match part {
        Part::Part1 => solve_part_1(input_file_path),
        Part::Part2 => solve_part_2(input_file_path),
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
            cum_sum = 0;
        } else {
            let line_int: i32 = line_unwrapped.parse().expect("Cannot parse line to int.");
            cum_sum += line_int;
            max_sum = max(max_sum, cum_sum);
        }
    }

    println!("{}", max_sum);
    max_sum
}

fn solve_part_2(input_file_path: &Path) -> i32 {
    let lines: Vec<String> = read_to_string(input_file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let n_lines = lines.len() - 1;
    // Reverse() switches default MaxHeap behaviour to MinHeap.
    // Interestingly, in Rust's implementation
    // push() is O(1) and pop() is O(log N).
    let mut min_heap = BinaryHeap::new();
    let mut cum_sum: i32 = 0;

    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() || i == n_lines {
            if i == n_lines {
                let n: i32 = line.parse().unwrap();
                cum_sum += n;
            }
            if min_heap.len() < 3 {
                min_heap.push(Reverse(cum_sum));
            } else {
                if cum_sum > min_heap.peek().unwrap().0 {
                    min_heap.pop();
                    min_heap.push(Reverse(cum_sum));
                }
            }
            cum_sum = 0;
        } else {
            let n: i32 = line.parse().unwrap();
            cum_sum += n;
        }
    }

    let mut total = 0;
    for cum_sum in min_heap {
        // .0 here extracts i32 from Reverse(i32).
        total += cum_sum.0;
    }

    println!("{}", total);
    total
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

    #[test]
    fn test_solve_part_2() {
        // Part 1 and 2 share the same test input.
        let input_file_path = common::tests::data_path("day_01_part_1.txt");
        assert_eq!(solve_part_2(input_file_path.as_path()), 45000);
    }
}
