use std::collections::{BinaryHeap, HashSet};

use aoc_macros::aoc;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn perp(&self) -> [Self; 2] {
        match self {
            Left => [Up, Down],
            Right => [Up, Down],
            Up => [Left, Right],
            Down => [Left, Right],
        }
    }
    fn next_part_1(
        &self,
        (y, x): (usize, usize),
        (b_y, b_x): (usize, usize),
    ) -> Vec<(usize, usize)> {
        match self {
            Left => (x.saturating_sub(3)..x)
                .rev()
                .map(move |x| (y, x))
                .collect(),
            Right => (x + 1..=x + 3)
                .take_while(|x| *x <= b_x)
                .map(move |x| (y, x))
                .collect(),
            Up => (y.saturating_sub(3)..y)
                .rev()
                .map(move |y| (y, x))
                .collect(),
            Down => (y + 1..=y + 3)
                .take_while(|y| *y <= b_y)
                .map(move |y| (y, x))
                .collect(),
        }
    }

    fn next_part_2(
        &self,
        (y, x): (usize, usize),
        (b_y, b_x): (usize, usize),
    ) -> Vec<(usize, usize)> {
        match self {
            Left => (x.saturating_sub(10)..x)
                .rev()
                .map(move |x| (y, x))
                .collect(),
            Right => (x + 1..=x + 10)
                .take_while(|x| *x <= b_x)
                .map(move |x| (y, x))
                .collect(),
            Up => (y.saturating_sub(10)..y)
                .rev()
                .map(move |y| (y, x))
                .collect(),
            Down => (y + 1..=y + 10)
                .take_while(|y| *y <= b_y)
                .map(move |y| (y, x))
                .collect(),
        }
    }
}

use Direction::*;

fn shortest_path(map: &[Vec<u32>], part_2: bool) -> u32 {
    let end = (map.len() - 1, map[0].len() - 1);
    let mut queue = BinaryHeap::new();
    queue.push((0_isize, Right, (0, 0)));
    queue.push((0, Left, (0, 0)));
    let mut visited = HashSet::new();
    let mut res = 0;
    while let Some((dist, dir, (y, x))) = queue.pop() {
        if (y, x) == end {
            res = dist;
            break;
        }
        if visited.contains(&(dir, (y, x))) {
            continue;
        }
        visited.insert((dir, (y, x)));
        let mut new_dist = dist;
        let next = match part_2 {
            false => dir.next_part_1((y, x), end),
            true => dir.next_part_2((y, x), end),
        };
        if part_2 {
            next.iter()
                .take(3)
                .for_each(|&(n_y, n_x)| new_dist -= map[n_y][n_x] as isize);
        }
        for &(n_y, n_x) in next.iter().skip(3) {
            new_dist -= map[n_y][n_x] as isize;
            for new_dir in dir.perp() {
                if visited.contains(&(new_dir, (n_y, n_x))) {
                    continue;
                }
                queue.push((new_dist, new_dir, (n_y, n_x)));
            }
        }
    }
    -res as u32
}

#[aoc(test = "102")]
fn part_1(inp: &str) -> u32 {
    let map: Vec<Vec<_>> = inp
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    shortest_path(&map, false)
}

#[aoc(test = "94")]
fn part_2(inp: &str) -> u32 {
    let map: Vec<Vec<_>> = inp
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    shortest_path(&map, true)
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
