use std::collections::HashSet;
use std::fs;
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

struct RopeEnd {
    coord: Coord,
    prev_coord: Coord,
}

impl RopeEnd {
    fn new() -> RopeEnd {
        RopeEnd {
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
    fn adjacent(&self, tail: &RopeEnd) -> bool {
        if (self.coord.x - tail.coord.x).abs() > 1 || (self.coord.y - tail.coord.y).abs() > 1 {
            return false;
        } else {
            return true;
        }
    }
    fn drag(&self, tail: &mut RopeEnd) -> () {
        if !self.adjacent(tail) {
            tail.coord.x = self.prev_coord.x;
            tail.coord.y = self.prev_coord.y;
        }
    }
}

pub fn solve(input_file_path: &Path, part: Part) -> () {
    match part {
        Part::Part1 => solve_part_1(input_file_path),
        Part::Part2 => panic!("Solution to part 2 is not implemented yet."),
    };
}

fn solve_part_1(input_file_path: &Path) -> u32 {
    let binding = fs::read_to_string(input_file_path).unwrap();
    let input = binding
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut head = RopeEnd::new();
    let mut tail = RopeEnd::new();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests;

    #[test]
    fn test_solve_part_1() {
        let input_file_path = tests::data_path("day_09_part_1.txt");
        assert_eq!(solve_part_1(input_file_path.as_path()), 13);
    }
}
