use std::collections::HashMap;

use aoc_macros::aoc;

#[derive(Hash, PartialEq, Eq, Clone)]
enum Rock {
    None,
    Circle,
    Square,
}

impl From<char> for Rock {
    fn from(value: char) -> Self {
        match value {
            'O' => Rock::Circle,
            '#' => Rock::Square,
            _ => Rock::None,
        }
    }
}

fn north(rows: &mut [Vec<Rock>]) {
    for x in 0..rows[0].len() {
        let mut height = 0;
        for y in 0..rows.len() {
            match rows[y][x] {
                Rock::Circle => {
                    if y != height {
                        rows[height][x] = Rock::Circle;
                        rows[y][x] = Rock::None;
                    }
                    height += 1;
                }
                Rock::Square => {
                    height = y + 1;
                }
                Rock::None => continue,
            }
        }
    }
}

fn south(rows: &mut [Vec<Rock>]) {
    for x in 0..rows[0].len() {
        let mut height = rows.len() - 1;
        for y in (0..rows.len()).rev() {
            match rows[y][x] {
                Rock::Circle => {
                    if height != y {
                        rows[height][x] = Rock::Circle;
                        rows[y][x] = Rock::None;
                    }
                    height = height.saturating_sub(1);
                }
                Rock::Square => {
                    height = y.saturating_sub(1);
                }
                Rock::None => continue,
            }
        }
    }
}

fn east(rows: &mut [Vec<Rock>]) {
    for y in 0..rows.len() {
        let mut width = rows[0].len() - 1;
        for x in (0..rows[0].len()).rev() {
            match rows[y][x] {
                Rock::Circle => {
                    if x != width {
                        rows[y][width] = Rock::Circle;
                        rows[y][x] = Rock::None;
                    }
                    width = width.saturating_sub(1);
                }
                Rock::Square => {
                    width = x.saturating_sub(1);
                }
                Rock::None => continue,
            }
        }
    }
}

fn west(rows: &mut [Vec<Rock>]) {
    for y in 0..rows.len() {
        let mut width = 0;
        for x in 0..rows[0].len() {
            match rows[y][x] {
                Rock::Circle => {
                    if x != width {
                        rows[y][width] = Rock::Circle;
                        rows[y][x] = Rock::None;
                    }

                    width += 1;
                }
                Rock::Square => {
                    width = x + 1;
                }
                Rock::None => continue,
            }
        }
    }
}

fn count_north(rows: &[Vec<Rock>]) -> u32 {
    let mut sum = 0;
    for x in 0..rows[0].len() {
        for y in 0..rows.len() {
            match rows[y][x] {
                Rock::Circle => {
                    sum += rows.len() - y;
                }
                _ => continue,
            }
        }
    }
    sum as u32
}

#[aoc(test = "136")]
fn part_1(inp: &str) -> u32 {
    let mut rows: Vec<Vec<_>> = inp
        .lines()
        .map(|line| line.chars().map(Rock::from).collect())
        .collect();
    north(&mut rows);
    count_north(&rows)
}

#[aoc(test = "64")]
fn part_2(inp: &str) -> u32 {
    let mut rows: Vec<Vec<_>> = inp
        .lines()
        .map(|l| l.chars().map(Rock::from).collect())
        .collect();
    let mut it = 0;
    let mut cache: HashMap<Vec<Vec<Rock>>, u64> = HashMap::new();
    while it < 1000000000 {
        north(&mut rows);
        west(&mut rows);
        south(&mut rows);
        east(&mut rows);
        it += 1;
        if let Some(x) = cache.get(&rows) {
            let cycle = it - x;
            let skipped = (1000000000 - it) / cycle;
            it += skipped * cycle;
        }
        cache.insert(rows.clone(), it);
    }
    count_north(&rows)
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
