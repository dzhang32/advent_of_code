use std::fs::read_to_string;
use std::path::Path;

use id_tree::InsertBehavior::{AsRoot, UnderNode};
use id_tree::{Node, NodeId, Tree};

use crate::Part;

pub fn solve(input_file_path: &Path, part: Part) -> () {
    match part {
        Part::Part1 => solve_part_1(input_file_path),
        Part::Part2 => solve_part_2(input_file_path),
    };
}

fn solve_part_1(input_file_path: &Path) -> i32 {
    let binding = read_to_string(input_file_path).unwrap();
    let input = binding
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let tree = generate_file_tree(input);
    let recursive_file_sizes = calculate_recursive_file_sizes(&tree);

    let mut total = 0;
    for s in recursive_file_sizes {
        if s <= 100000 {
            total += s;
        }
    }

    println!("{}", total);
    total
}

fn solve_part_2(input_file_path: &Path) -> i32 {
    let binding = read_to_string(input_file_path).unwrap();
    let input = binding
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let tree = generate_file_tree(input);
    let recursive_file_sizes = calculate_recursive_file_sizes(&tree);

    let required_free_space = 30000000 - (70000000 - recursive_file_sizes[0]);
    let mut to_delete = i32::MAX;
    for &s in &recursive_file_sizes {
        if s >= required_free_space && s < to_delete {
            to_delete = s;
        }
    }

    println!("{}", to_delete);
    to_delete
}

fn generate_file_tree(input: Vec<Vec<&str>>) -> Tree<i32> {
    // Creating trees from scratch in Rust is tricky, due to ownership rules:
    // https://users.rust-lang.org/t/need-help-creating-a-tree-node-struct/79866/2
    // Here, we will use the library id_tree.
    let mut tree: Tree<i32> = Tree::new();

    // Create a dummy root node to simplify downstream logic.
    let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
    let mut curr_node_id = root_id;
    let mut total_file_size = 0;

    let mut i: usize = 0;

    while i < input.len() - 1 {
        if input[i][1] == "ls" {
            // Calculate the file size total within the dir.
            while input[i + 1][0] != "$" {
                i += 1;
                match input[i][0].parse::<i32>() {
                    Ok(file_size) => total_file_size += file_size,
                    Err(_) => {}
                };
                if i == input.len() - 1 {
                    break;
                }
            }
            curr_node_id = tree
                .insert(Node::new(total_file_size), UnderNode(&curr_node_id))
                .unwrap();
            // Reset to 0 for the next dir.
            total_file_size = 0;
        } else if input[i][1] == "cd" && input[i][2] == ".." {
            curr_node_id = tree.get(&curr_node_id).unwrap().parent().unwrap().clone();
        }

        i += 1;
    }

    tree
}

fn calculate_recursive_file_sizes(tree: &Tree<i32>) -> Vec<i32> {
    // The root of the tree is a dummy node.
    let root = tree.get(tree.root_node_id().unwrap()).unwrap();
    let mut stack = Vec::new();
    stack.push(&root.children()[0]);
    let mut recursive_file_sizes = Vec::new();

    // Traverse the tree using DFS.
    while stack.len() != 0 {
        let current_node = stack.pop().unwrap();
        recursive_file_sizes.push(get_recursive_file_size(current_node, tree));
        for child in tree.get(&current_node).unwrap().children() {
            stack.push(child);
        }
    }
    recursive_file_sizes
}

fn get_recursive_file_size(node_id: &NodeId, tree: &Tree<i32>) -> i32 {
    let mut recursive_file_size = 0;
    let mut stack = vec![node_id];

    while stack.len() != 0 {
        let current_node = tree.get(&stack.pop().unwrap()).unwrap();
        recursive_file_size += current_node.data();
        for child in current_node.children() {
            stack.push(child);
        }
    }

    recursive_file_size
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests;

    #[test]
    fn test_solve_part_1() {
        let input_file_path = tests::data_path("day_07_part_1.txt");
        assert_eq!(solve_part_1(input_file_path.as_path()), 95437);
    }

    #[test]
    fn test_solve_part_2() {
        let input_file_path = tests::data_path("day_07_part_1.txt");
        assert_eq!(solve_part_2(input_file_path.as_path()), 24933642);
    }
}
