use std::{fmt::{Debug}, collections::HashMap};

use aoc_macros::aoc;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Spring {
    Operational,
    Damaged,
    Unknown
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => unreachable!("Unknown spring character")
        }
    }
}

impl Debug for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Spring::Operational => ".",
            Spring::Damaged => "#",
            Spring::Unknown => "?",
        })
    }
}

type Cache = HashMap<(Vec<Spring>, Vec<i32>, bool, bool), u64>;

fn count_permutations(cache: &mut Cache, springs: &mut [Spring], config: &mut [i32], space: bool, first: bool) -> u64 {
    if config.len() == 1 && config[0] == 0 && !springs.contains(&Spring::Damaged) {
        return 1;
    }
    if springs.len() == 0 {
        return 0;
    }
    if config[0] < 0 {
        return 0;
    }
    if let Some(x) = cache.get(&(springs.to_vec(), config.to_vec(), space, first)) {
        return *x
    }
    let permutations = match springs[0] {
        Spring::Operational => {
            let permutations = count_permutations(cache, &mut springs[1..], config, true, first);
            permutations
        }
        Spring::Damaged => {
            if space && !first {
                if config[0] > 0 || config.len() <= 1 {
                    return 0;
                }
                config[1] -= 1;
                let permutations = count_permutations(cache, &mut springs[1..], &mut config[1..], false, false);
                config[1] += 1;
                permutations
            } else {
                config[0] -= 1;
                let permutations = count_permutations(cache, &mut springs[1..], config, false, false);
                config[0] += 1;
                permutations
            }
        },
        Spring::Unknown => {
            springs[0] = Spring::Operational;
            let permutations_1 = count_permutations(cache, springs, config, space, first);
            springs[0] = Spring::Damaged;
            let permutations_2 = count_permutations(cache, springs, config, space, first);
            springs[0] = Spring::Unknown;
            permutations_1 + permutations_2
        },
    };
    cache.insert((springs.to_vec(), config.to_vec(), space, first), permutations);
    permutations
}

#[aoc(test = "21")]
fn part_1(inp: &str) -> u64 {
    let mut cache = HashMap::new();
    let mut sum = 0;
    for line in inp.lines() {
        let (spring, config) = line.split_once(' ').unwrap();
        let mut spring: Vec<_> = spring.chars().map(Spring::from).collect();
        let mut config: Vec<i32> = config.split(',').filter_map(|c| c.parse().ok()).collect();
        sum += count_permutations(&mut cache, &mut spring, &mut config, true, true)
    }
    sum
}

#[aoc(test = "525152")]
fn part_2(inp: &str) -> u64 {
    let mut sum = 0;
    let mut cache = HashMap::new();
    for line in inp.lines() {
        let (spring, config) = line.split_once(' ').unwrap();
        let spring = format!("{}?{}?{}?{}?{}", spring, spring, spring, spring, spring);
        let config = format!("{},{},{},{},{}", config, config, config, config, config);
        let mut spring: Vec<_> = spring.chars().map(Spring::from).collect();
        let mut config: Vec<i32> = config.split(',').filter_map(|c| c.parse().ok()).collect();
        sum += count_permutations(&mut cache, &mut spring, &mut config, true, true)
    }
    sum
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
