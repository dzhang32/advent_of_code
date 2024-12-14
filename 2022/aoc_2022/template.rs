use std::path::Path;

use crate::Part;

pub fn solve(input_file_path: &Path, part: Part) -> () {
    match part {
        Part::Part1 => solve_part_1(input_file_path),
        Part::Part2 => panic!("Solution to part 2 is not implemented yet."),
    };
}

fn solve_part_1(input_file_path: &Path) -> () {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests;

    #[test]
    fn test_solve_part_1() {
        let input_file_path = tests::data_path("day_01_part_1.txt");
        assert_eq!(solve_part_1(input_file_path.as_path()), ());
    }
}
