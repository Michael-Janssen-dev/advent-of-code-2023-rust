use std::{collections::HashMap};

use aoc_macros::aoc;

use num::Integer;

#[aoc(test = "6", part = 1)]
fn part_1(inp: &str) -> u32 {
    let (path, nodes) = inp.split_once("\n\n").unwrap();
    let mut trees: HashMap<&str, (&str, &str)> = HashMap::new();
    for node in nodes.lines() {
        let (parent, children) = node.split_once(" = ").unwrap();
        let (left, right) = children.split_once(", ").unwrap();
        let left = &left[1..];
        let right = &right[..3];
        trees.insert(parent, (left, right));
    }
    let mut count = 1;
    let mut current_node = "AAA";
    for p in path.chars().cycle() {
        let entry = trees.get(current_node).unwrap();
        current_node = match p {
            'L' => entry.0,
            'R' => entry.1,
            _ => unreachable!("Wrong character in path!"),
        };
        if current_node == "ZZZ" {
            break;
        }
        count += 1;
    }
    count
}

#[aoc(test = "6", part = 2)]
fn part_2(inp: &str) -> i128 {
    let (path, nodes) = inp.split_once("\n\n").unwrap();
    let mut trees: HashMap<&str, (&str, &str)> = HashMap::new();
    for node in nodes.lines() {
        let (parent, children) = node.split_once(" = ").unwrap();
        let (left, right) = children.split_once(", ").unwrap();
        let left = &left[1..];
        let right = &right[..3];
        trees.insert(parent, (left, right));
    }
    let mut current_nodes: Vec<&str> = trees
        .keys()
        .filter(|t| t.ends_with('A')).copied()
        .collect();
    let mut c: i128 = 1;
    let mut done = Vec::new();
    for p in path.chars().cycle() {
        if done.iter().len() == current_nodes.iter().len() {
            break;
        }
        let mut new_nodes = Vec::new();
        for node in &current_nodes {
            let entry = trees.get(node).unwrap();
            let node = match p {
                'L' => entry.0,
                'R' => entry.1,
                _ => unreachable!("Wrong character in path!"),
            };
            new_nodes.push(node);
            if node.chars().nth(2).unwrap() == 'Z' {
                done.push(c);
            }
        }
        current_nodes = new_nodes;
        c += 1;
    }
    done.into_iter().reduce(|a, b| a.lcm(&b)).unwrap()
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
