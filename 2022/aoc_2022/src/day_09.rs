use std::collections::HashSet;
use std::fs::{self, read_to_string};
use std::path::Path;

use crate::Part;

// Clone - avoid moving ownership when adding coords to HashSet.
// Debug - print coords.
// PartialEq, Eq, Hash - allow storage of coords in HashSet.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct RopeKnot {
    coord: Coord,
    prev_coord: Coord,
}

impl RopeKnot {
    fn new() -> RopeKnot {
        RopeKnot {
            coord: Coord { x: 0, y: 0 },
            prev_coord: Coord { x: 0, y: 0 },
        }
    }
    fn step(&mut self, direction: &str) -> () {
        self.prev_coord.x = self.coord.x;
        self.prev_coord.y = self.coord.y;
        match direction {
            "R" => self.coord.x += 1,
            "L" => self.coord.x -= 1,
            "U" => self.coord.y += 1,
            "D" => self.coord.y -= 1,
            _ => {}
        }
    }
    fn adjacent(&self, next_rope_knot: &RopeKnot) -> bool {
        if (self.coord.x - next_rope_knot.coord.x).abs() > 1
            || (self.coord.y - next_rope_knot.coord.y).abs() > 1
        {
            return false;
        } else {
            return true;
        }
    }
    fn drag(&self, tail: &mut RopeKnot) -> () {
        if !self.adjacent(tail) {
            tail.coord.x = self.prev_coord.x;
            tail.coord.y = self.prev_coord.y;
        }
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
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut head = RopeKnot::new();
    let mut tail = RopeKnot::new();
    let mut visited = HashSet::new();

    for instruction in input {
        let direction = instruction[0];
        let n: usize = instruction[1].parse().unwrap();

        for _ in 0..n {
            head.step(direction);
            head.drag(&mut tail);
            visited.insert(tail.coord.clone());
        }
    }

    println!("{}", visited.len());
    visited.len() as u32
}

fn solve_part_2(input_file_path: &Path) -> u32 {
    let binding = read_to_string(input_file_path).unwrap();
    let input = binding
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut rope = Vec::with_capacity(10);
    for _ in 0..10 {
        rope.push(RopeKnot::new());
    }
    let mut visited = HashSet::new();

    for instruction in input {
        let direction = instruction[0];
        let steps: usize = instruction[1].parse().unwrap();

        for _ in 0..steps {
            // First, move the head.
            rope[0].step(direction);

            // Then, drag the remaining knots.
            for i in 0..9 {
                if !rope[i].adjacent(&rope[i + 1]) {
                    let diff = (
                        rope[i].coord.x - rope[i + 1].coord.x,
                        rope[i].coord.y - rope[i + 1].coord.y,
                    );
                    let (dx, dy) = match diff {
                        // need to move up/left/down/right
                        (0, 2) => (0, 1),
                        (0, -2) => (0, -1),
                        (2, 0) => (1, 0),
                        (-2, 0) => (-1, 0),
                        // need to move to the right diagonally
                        (2, 1) => (1, 1),
                        (2, -1) => (1, -1),
                        // need to move to the left diagonally
                        (-2, 1) => (-1, 1),
                        (-2, -1) => (-1, -1),
                        // need to move up/down diagonally
                        (1, 2) => (1, 1),
                        (-1, 2) => (-1, 1),
                        (1, -2) => (1, -1),
                        (-1, -2) => (-1, -1),
                        // 🆕 need to move diagonally
                        (-2, -2) => (-1, -1),
                        (-2, 2) => (-1, 1),
                        (2, -2) => (1, -1),
                        (2, 2) => (1, 1),
                        _ => panic!(""),
                    };
                    rope[i + 1].coord.x += dx;
                    rope[i + 1].coord.y += dy;
                }
                if i == 8 {
                    visited.insert(rope[i + 1].coord.clone());
                }
            }
        }
    }

    println!("{}", visited.len());
    visited.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests;
    use rstest::rstest;

    #[test]
    fn test_solve_part_1() {
        let input_file_path = tests::data_path("day_09_part_1.txt");
        assert_eq!(solve_part_1(input_file_path.as_path()), 13);
    }

    #[rstest]
    #[case("day_09_part_1.txt", 1)]
    #[case("day_09_part_2.txt", 36)]
    fn test_solve_part_2(#[case] input_path: &str, #[case] expected: u32) {
        let input_file_path = tests::data_path(input_path);
        assert_eq!(solve_part_2(input_file_path.as_path()), expected);
    }
}
