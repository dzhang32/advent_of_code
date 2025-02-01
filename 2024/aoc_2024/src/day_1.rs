use crate::utils::data_path;
use std::collections::{BinaryHeap, HashMap};
use std::{fs::File, io::{BufRead, BufReader}, path::Path};
use std::cmp::Reverse;

pub fn solve(day: i32, part: i32) -> Result<(), String> {
    match part {
        1 => solve_part_1(&data_path(day, part)),
        2 => solve_part_2(&data_path(day, part)),
        _ => return Err("Part must be 1 or 2.".to_string()),
    };
    Ok(())
}

fn solve_part_1(input_file_path: &Path) -> i32 {
    let file = File::open(input_file_path).unwrap();
    let reader = BufReader::new(file);

    let mut min_heap_1 = BinaryHeap::new();
    let mut min_heap_2 = BinaryHeap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.split_whitespace().collect::<Vec<&str>>();

        let int_1 = line[0].parse::<i32>().unwrap();
        let int_2 = line[1].parse::<i32>().unwrap();

        min_heap_1.push(Reverse(int_1));
        min_heap_2.push(Reverse(int_2));
    }

    let mut total_dist = 0;
    let n = min_heap_1.len();

    for _ in 0..n {
        let dist = min_heap_1.pop().unwrap().0 - min_heap_2.pop().unwrap().0;

        total_dist += dist.abs()
    }

    println!("{:?}", total_dist);

    total_dist
    
}

fn solve_part_2(input_file_path: &Path) -> i32 {
    let file = File::open(input_file_path).unwrap();
    let reader = BufReader::new(file);

    let mut nums = Vec::new();  
    let mut seen = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.split_whitespace().collect::<Vec<&str>>();

        let int_1 = line[0].parse::<i32>().unwrap();
        let int_2 = line[1].parse::<i32>().unwrap();

        nums.push(int_1);
        let n = seen.entry(int_2).or_insert(0);
        *n += 1;
    }

    let mut total: i32 = 0;

    for n in nums {
        if seen.contains_key(&n) {
            total += n * seen.get(&n).unwrap();
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1() {
        let input_file_path = data_path(1, 1);
        assert_eq!(solve_part_1(&input_file_path), 1110981)
    }

    #[test]
    fn test_solve_part_2() {
        let input_file_path = data_path(1, 1);
        assert_eq!(solve_part_2(&input_file_path), 24869388)
    }
}
