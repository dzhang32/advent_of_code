use std::fs::read_to_string;
use std::path::Path;

use crate::Part;

pub fn solve(input_file_path: &Path, part: Part) -> () {
    match part {
        Part::Part1 => solve_part_1(input_file_path),
        // Rust complains when arms return different types.
        // Here, part 1 returns an i32 and part 2 a Vec<String>.
        Part::Part2 => {
            solve_part_2(input_file_path);
            1
        }
    };
}

#[derive(Debug)]
enum Command {
    Add(i32),
    Next,
}

fn solve_part_1(input_file_path: &Path) -> i32 {
    let binding = read_to_string(input_file_path).unwrap();
    let input = binding
        .lines()
        .map(|l| {
            let l_split = l.split_whitespace().collect::<Vec<&str>>();
            match l_split[0] {
                "addx" => Command::Add(l_split[1].parse::<i32>().unwrap()),
                "noop" => Command::Next,
                other => panic!("Unknown command: {:?}", other),
            }
        })
        .collect::<Vec<Command>>();

    let mut current_signal = 1;
    let mut current_cycle = 1;

    let mut important_cycles = vec![220, 180, 140, 100, 60];
    let mut looking_for = 20;

    let mut signals: Vec<i32> = Vec::with_capacity(important_cycles.len());

    for command in input {
        match command {
            Command::Add(value) => {
                // Check if important cycle is reached before the 2 cycles
                // it takes to execute the Add command completes.
                current_cycle += 1;
                if looking_for <= current_cycle {
                    looking_for = store_signal_strength(
                        current_signal,
                        current_cycle,
                        &mut signals,
                        &mut important_cycles,
                    );
                }

                current_signal += value;
                current_cycle += 1;
            }
            Command::Next => current_cycle += 1,
        };

        if looking_for <= current_cycle {
            looking_for = store_signal_strength(
                current_signal,
                current_cycle,
                &mut signals,
                &mut important_cycles,
            );
        }
    }

    let sum_signals: i32 = signals.iter().sum();
    println!("{:?}", sum_signals);
    sum_signals
}

fn store_signal_strength(
    current_signal: i32,
    current_cycle: i32,
    signals: &mut Vec<i32>,
    important_cycles: &mut Vec<i32>,
) -> i32 {
    signals.push(current_signal * current_cycle);
    important_cycles.pop().unwrap_or(i32::MAX)
}

struct Sprite {
    position: i32,
}

impl Sprite {
    fn new() -> Sprite {
        Sprite { position: 1 }
    }
    fn touching(&self, cycle: i32) -> bool {
        let cycle_mod_40 = cycle % 40;
        self.position - 1 <= cycle_mod_40 && cycle_mod_40 <= self.position + 1
    }
    fn draw(&self, cycle: i32, drawing: &mut String) -> () {
        if self.touching(cycle) {
            drawing.push('#');
        } else {
            drawing.push('.');
        }
    }
}

fn solve_part_2(input_file_path: &Path) -> Vec<String> {
    let binding = read_to_string(input_file_path).unwrap();
    let input = binding
        .lines()
        .map(|l| {
            let l_split = l.split_whitespace().collect::<Vec<&str>>();
            match l_split[0] {
                "addx" => Command::Add(l_split[1].parse::<i32>().unwrap()),
                "noop" => Command::Next,
                other => panic!("Unknown command: {:?}", other),
            }
        })
        .collect::<Vec<Command>>();

    let mut sprite = Sprite::new();
    let mut cycle = 0;
    let mut drawing = String::new();

    for command in input {
        match command {
            Command::Add(value) => {
                sprite.draw(cycle, &mut drawing);
                cycle += 1;
                sprite.draw(cycle, &mut drawing);
                cycle += 1;
                sprite.position += value;
            }
            Command::Next => {
                sprite.draw(cycle, &mut drawing);
                cycle += 1;
            }
        }
    }

    let binding = drawing.chars().collect::<Vec<char>>();
    let drawing_rows = binding
        .chunks(40)
        .map(|c| c.into_iter().collect::<String>())
        .collect::<Vec<_>>();

    println!("{:#?}", drawing_rows);
    drawing_rows
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests;

    #[test]
    fn test_solve_part_1() {
        let input_file_path = tests::data_path("day_10_part_1.txt");
        assert_eq!(solve_part_1(input_file_path.as_path()), 13140);
    }

    #[test]
    fn test_solve_part_2() {
        let input_file_path = tests::data_path("day_10_part_1.txt");
        let expected_output = vec![
            String::from("##..##..##..##..##..##..##..##..##..##.."),
            String::from("###...###...###...###...###...###...###."),
            String::from("####....####....####....####....####...."),
            String::from("#####.....#####.....#####.....#####....."),
            String::from("######......######......######......####"),
            String::from("#######.......#######.......#######....."),
        ];
        assert_eq!(solve_part_2(input_file_path.as_path()), expected_output);
    }
}
