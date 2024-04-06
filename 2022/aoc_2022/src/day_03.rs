use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;

use crate::Part;

pub fn solve(input_file_path: &Path, part: Part) -> () {
    match part {
        Part::Part1 => solve_part_1(input_file_path),
        Part::Part2 => solve_part_2(input_file_path),
    };
}

fn solve_part_1(input_file_path: &Path) -> u32 {
    // Need to create binding, which lives longer than temp values
    // created within a method chain.
    // Otherwise, compiler complains that usage of input after chain is
    // referencing a freed value, therefore not memory safe.
    let binding: String = read_to_string(input_file_path).unwrap();
    let input: Vec<[&str; 2]> = binding
        .lines()
        .map(|line| {
            let mid_point = line.len() / 2;
            let first_chunk = &line[..mid_point];
            let second_chunk = &line[mid_point..];
            // Each element is a fixed size (2), so we can use an array.
            [first_chunk, second_chunk]
        })
        .collect();

    let mut total_score: u32 = 0;

    for backpack in &input {
        let section_2: HashSet<char> = backpack[1].chars().collect();
        let common = backpack[0]
            .chars()
            .find(|&x| section_2.contains(&x))
            .expect("No common characters found.");

        total_score += char_to_alphabet_value(common).expect("Value must be part of the alphabet.");
    }

    println!("{}", total_score);
    total_score
}

fn char_to_alphabet_value(c: char) -> Option<u32> {
    match c {
        'A'..='Z' => Some(c as u32 - 'A' as u32 + 27),
        'a'..='z' => Some(c as u32 - 'a' as u32 + 1),
        _ => None,
    }
}

fn solve_part_2(input_file_path: &Path) -> u32 {
    let binding = read_to_string(input_file_path).unwrap();
    let input = binding
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<Vec<&str>>>();

    let mut total: u32 = 0;

    for item in &input {
        let section_1: HashSet<char> = item[0].chars().collect();
        let section_2: HashSet<char> = item[1].chars().collect();
        let common_1_2 = section_1.intersection(&section_2).collect::<HashSet<_>>();
        let common = item[2]
            .chars()
            .find(|c| common_1_2.contains(c))
            .expect("No common characters found.");

        total += char_to_alphabet_value(common).expect("Value must be part of the alphabet.");
    }

    println!("{}", total);
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests;

    #[test]
    fn test_solve_part_1() {
        let input_file_path = tests::data_path("day_03_part_1.txt");
        assert_eq!(solve_part_1(input_file_path.as_path()), 157);
    }

    #[test]
    fn test_solve_part_2() {
        let input_file_path = tests::data_path("day_03_part_1.txt");
        assert_eq!(solve_part_2(input_file_path.as_path()), 70);
    }

    #[test]
    fn test_char_to_alphabet_value() {
        assert_eq!(char_to_alphabet_value('A'), Some(27));
        assert_eq!(char_to_alphabet_value('a'), Some(1));
        assert_eq!(char_to_alphabet_value('Z'), Some(52));
        assert_eq!(char_to_alphabet_value('z'), Some(26));
        assert_eq!(char_to_alphabet_value('0'), None);
    }
}
