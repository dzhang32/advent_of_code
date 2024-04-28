use std::fs::read_to_string;
use std::path::PathBuf;

use crate::{Config, Part};

#[derive(Debug)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

struct Submarine {
    horizontal: i32,
    vertical: i32,
    aim: i32,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine {
            horizontal: 0,
            vertical: 0,
            aim: 0,
        }
    }
    fn pilot_part_1(&mut self, command: Command) -> () {
        match command {
            Command::Forward(magnitude) => self.horizontal += magnitude,
            Command::Down(magnitude) => self.vertical += magnitude,
            Command::Up(magnitude) => self.vertical -= magnitude,
        }
    }
    fn pilot_part_2(&mut self, command: Command) -> () {
        match command {
            Command::Forward(magnitude) => {
                self.horizontal += magnitude;
                self.vertical += magnitude * self.aim;
            }
            Command::Down(magnitude) => self.aim += magnitude,
            Command::Up(magnitude) => self.aim -= magnitude,
        }
    }
    fn position(&self) -> i32 {
        self.horizontal * self.vertical
    }
}

impl Command {
    fn from(v: Vec<&str>) -> Command {
        let command = v[0];
        let magnitude = v[1].parse::<i32>().unwrap();
        match command {
            "forward" => Command::Forward(magnitude),
            "down" => Command::Down(magnitude),
            "up" => Command::Up(magnitude),
            _ => panic!("Command not accounted for {}", command),
        }
    }
}

pub fn solve(config: Config) -> () {
    match config.part {
        Part::One => solve_part_1(config.input_path),
        Part::Two => solve_part_2(config.input_path),
    };
}

fn parse_input(input_path: PathBuf) -> Vec<Command> {
    let binding = read_to_string(input_path).unwrap();
    let input = binding
        .lines()
        .map(|l| Command::from(l.split_whitespace().collect::<Vec<&str>>()))
        .collect::<Vec<Command>>();
    input
}

fn solve_part_1(input_path: PathBuf) -> i32 {
    let input = parse_input(input_path);
    let mut submarine = Submarine::new();
    for command in input {
        submarine.pilot_part_1(command)
    }
    let position = submarine.position();

    println!("{}", position);
    position
}

fn solve_part_2(input_path: PathBuf) -> i32 {
    let input = parse_input(input_path);
    let mut submarine = Submarine::new();
    for command in input {
        submarine.pilot_part_2(command)
    }
    let position = submarine.position();

    println!("{}", position);
    position
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests;

    #[test]
    fn test_solve_part_1() {
        let input_path = tests::data_path("day_02_part_1.txt");
        assert_eq!(solve_part_1(input_path), 150);
    }

    #[test]
    fn test_solve_part_2() {
        let input_path = tests::data_path("day_02_part_1.txt");
        assert_eq!(solve_part_2(input_path), 900);
    }
}
