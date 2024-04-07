use std::cmp::max;
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

fn solve_part_1(input_file_path: &Path) -> i32 {
    let input = read_to_string(input_file_path).unwrap();
    let result = find_first_n_unique_characters(&input, 4);

    println!("{}", result);
    result
}

fn solve_part_2(input_file_path: &Path) -> i32 {
    let input = read_to_string(input_file_path).unwrap();
    let result = find_first_n_unique_characters(&input, 14);

    println!("{}", result);
    result
}

fn find_first_n_unique_characters(s: &str, n: i32) -> i32 {
    // Returns index (+1) of the first occurence of
    // N unique characters within a string.
    let mut seen: HashMap<char, i32> = HashMap::new();
    let mut slow: i32 = 0;
    for (i, c) in s.chars().enumerate() {
        if seen.contains_key(&c) {
            slow = max(slow, seen.get(&c).unwrap() + 1);
        }

        let i_int = i as i32;
        if i_int - slow >= n - 1 {
            return i_int + 1;
        }

        seen.insert(c, i_int);
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests;
    use rstest::rstest;

    #[test]
    fn test_solve_part_1() {
        let input_file_path = tests::data_path("day_06_part_1.txt");
        assert_eq!(solve_part_1(input_file_path.as_path()), 7);
    }

    #[test]
    fn test_solve_part_2() {
        let input_file_path = tests::data_path("day_06_part_1.txt");
        assert_eq!(solve_part_2(input_file_path.as_path()), 19);
    }

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    #[case("aaaa", -1)]
    fn test_find_first_4_unique_characters(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(find_first_n_unique_characters(input, 4), expected)
    }
}
