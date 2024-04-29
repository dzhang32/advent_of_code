use std::fs::read_to_string;
use std::path::PathBuf;

use crate::{Config, Part};

#[derive(Debug)]
struct BitFreq {
    one: u32,
    zero: u32,
}

impl BitFreq {
    fn new() -> BitFreq {
        BitFreq { one: 0, zero: 0 }
    }
    fn count(&mut self, bit: char) -> () {
        match bit {
            '1' => self.one += 1,
            '0' => self.zero += 1,
            _ => panic!("Unexpected value for bit"),
        }
    }
    fn gamma(&self) -> char {
        if self.one > self.zero {
            return '1';
        }
        '0'
    }
    fn episilon(&self) -> char {
        if self.one < self.zero {
            return '1';
        }
        '0'
    }
    fn oxygen(&self) -> char {
        if self.one >= self.zero {
            return '1';
        }
        '0'
    }
    fn co2(&self) -> char {
        if self.one < self.zero {
            return '1';
        }
        '0'
    }
}

pub fn solve(config: Config) -> () {
    match config.part {
        Part::One => solve_part_1(config.input_path),
        Part::Two => solve_part_2(config.input_path),
    };
}

fn parse_input(input_path: PathBuf) -> Vec<Vec<char>> {
    let binding = read_to_string(input_path).unwrap();
    let input = binding
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    input
}

fn solve_part_1(input_path: PathBuf) -> isize {
    let input = parse_input(input_path);
    let n = input[0].len();
    let mut bit_freqs = Vec::with_capacity(n);
    for _ in 0..n {
        bit_freqs.push(BitFreq::new())
    }
    for line in input {
        for i in 0..line.len() {
            bit_freqs[i].count(line[i])
        }
    }

    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    for bit_freq in bit_freqs {
        gamma_rate.push(bit_freq.gamma());
        epsilon_rate.push(bit_freq.episilon())
    }

    let gamma_rate = isize::from_str_radix(gamma_rate.as_str(), 2).unwrap();
    let epsilon_rate = isize::from_str_radix(epsilon_rate.as_str(), 2).unwrap();
    let power_consumption = gamma_rate * epsilon_rate;

    println!("{}", power_consumption);
    power_consumption
}

fn solve_part_2(input_path: PathBuf) -> isize {
    let input = parse_input(input_path);

    let oxygen_rate = calculate_oxygen_co2_rate(input.clone(), "oxygen");
    let co2_rate = calculate_oxygen_co2_rate(input.clone(), "co2");

    println!("{}", oxygen_rate * co2_rate);
    oxygen_rate * co2_rate
}

fn calculate_oxygen_co2_rate(input: Vec<Vec<char>>, oxygen_co2: &str) -> isize {
    let mut i = 0;
    let mut filtered = input.clone();
    while filtered.len() > 1 {
        let mut bit_freq = BitFreq::new();
        for line in &filtered {
            bit_freq.count(line[i]);
        }
        filtered = match oxygen_co2 {
            "oxygen" => filtered
                .into_iter()
                .filter(|x| x[i] == bit_freq.oxygen())
                .collect::<Vec<Vec<char>>>(),
            "co2" => filtered
                .into_iter()
                .filter(|x| x[i] == bit_freq.co2())
                .collect::<Vec<Vec<char>>>(),
            _ => panic!("Either 'oxygen'/'co2' must be supplied."),
        };
        i += 1;
    }

    isize::from_str_radix(filtered[0].iter().collect::<String>().as_str(), 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests;

    #[test]
    fn test_solve_part_1() {
        let input_path = tests::data_path("day_03_part_1.txt");
        assert_eq!(solve_part_1(input_path), 198);
    }

    #[test]
    fn test_solve_part_2() {
        let input_path = tests::data_path("day_03_part_1.txt");
        assert_eq!(solve_part_2(input_path), 230);
    }
}
