use std::cmp;
use std::fs;
use std::path::Path;

use crate::Part;

#[derive(Debug)]
struct Forest {
    grid: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

impl Forest {
    fn from(input: Vec<Vec<u32>>) -> Forest {
        Forest {
            height: input.len(),
            width: input[0].len(),
            grid: input,
        }
    }
    fn get(&self, x: usize, y: usize) -> u32 {
        self.grid[y][x]
    }
    fn set(&mut self, x: usize, y: usize, value: u32) -> () {
        self.grid[y][x] = value;
    }
    fn left(&self, x: usize, y: usize) -> u32 {
        self.grid[y][x - 1]
    }
    fn right(&self, x: usize, y: usize) -> u32 {
        self.grid[y][x + 1]
    }
    fn top(&self, x: usize, y: usize) -> u32 {
        self.grid[y - 1][x]
    }
    fn bottom(&self, x: usize, y: usize) -> u32 {
        self.grid[y + 1][x]
    }
}

pub fn solve(input_file_path: &Path, part: Part) -> () {
    match part {
        Part::Part1 => solve_part_1(input_file_path),
        Part::Part2 => solve_part_2(input_file_path),
    };
}

fn solve_part_1(input_file_path: &Path) -> u32 {
    let binding = fs::read_to_string(input_file_path).unwrap();
    let input = binding
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    // This solution should have a O(n) time complexity
    // with the trade off of bring memory intensive,
    // Create matrices that store the largest tree to the left/right/top/bottom.
    let default = Forest::from(input.clone());
    let mut left = Forest::from(input.clone());
    let mut right = Forest::from(input.clone());
    let mut top = Forest::from(input.clone());
    let mut bottom = Forest::from(input.clone());

    for y in 1..default.height - 1 {
        for x in 1..default.width - 1 {
            left.set(x, y, cmp::max(left.get(x, y), left.left(x, y)));
            top.set(x, y, cmp::max(top.get(x, y), top.top(x, y)));
        }
    }

    for y in (1..default.height - 1).rev() {
        for x in (1..default.height - 1).rev() {
            right.set(x, y, cmp::max(right.get(x, y), right.right(x, y)));
            bottom.set(x, y, cmp::max(bottom.get(x, y), bottom.bottom(x, y)));
        }
    }

    // Start with the baseline of all trees around the edge of the box;
    // these are always visible.
    let mut visible = (default.height as u32 * 2) + (default.width as u32 * 2) - 4;

    // Then, add the visible trees in the inner section of the forest.
    for y in 1..default.height - 1 {
        for x in 1..default.width - 1 {
            let curr = default.get(x, y);
            if curr > left.left(x, y)
                || curr > right.right(x, y)
                || curr > top.top(x, y)
                || curr > bottom.bottom(x, y)
            {
                visible += 1;
            }
        }
    }

    println!("{}", visible);
    visible
}

fn solve_part_2(input_file_path: &Path) -> u32 {
    let binding = fs::read_to_string(input_file_path).unwrap();
    let input = binding
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let default = Forest::from(input.clone());
    let mut max_scenic_score = u32::MIN;

    for y in 1..default.height - 1 {
        for x in 1..default.width - 1 {
            let (mut visible_left, mut visible_right, mut visible_top, mut visible_bottom) =
                (1, 1, 1, 1);
            for i in (2..=x).rev() {
                if default.get(i - 1, y) >= default.get(x, y) {
                    break;
                } else {
                    visible_left += 1;
                }
            }
            for i in x..default.width - 2 {
                if default.get(i + 1, y) >= default.get(x, y) {
                    break;
                } else {
                    visible_right += 1;
                }
            }
            for i in (2..=y).rev() {
                if default.get(x, i - 1) >= default.get(x, y) {
                    break;
                } else {
                    visible_top += 1;
                }
            }
            for i in y..default.height - 2 {
                if default.get(x, i + 1) >= default.get(x, y) {
                    break;
                } else {
                    visible_bottom += 1;
                }
            }

            let scenic_score = visible_left * visible_right * visible_top * visible_bottom;
            if max_scenic_score < scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    println!("{}", max_scenic_score);
    max_scenic_score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests;

    #[test]
    fn test_solve_part_1() {
        let input_file_path = tests::data_path("day_08_part_1.txt");
        assert_eq!(solve_part_1(input_file_path.as_path()), 21);
    }

    #[test]
    fn test_solve_part_2() {
        let input_file_path = tests::data_path("day_08_part_1.txt");
        assert_eq!(solve_part_2(input_file_path.as_path()), 8);
    }
}
