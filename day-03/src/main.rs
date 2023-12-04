use std::collections::HashSet;

use aoc_macros::aoc;

#[aoc(test = "4361")]
fn part_1(inp: &str) -> u32 {
    let mut sum = 0;
    let lines: Vec<Vec<char>> = inp.lines().map(|line| line.chars().collect()).collect();
    for y in 0..lines.len() {
        let mut x = 0;
        while x < lines[0].len() {
            if lines[y][x].is_ascii_digit() {
                let mut digits = vec![lines[y][x].to_digit(10).unwrap()];
                for i in x + 1..lines[0].len() {
                    if lines[y][i].is_ascii_digit() {
                        digits.push(lines[y][i].to_digit(10).unwrap())
                    } else {
                        break;
                    }
                }
                for i in y.saturating_sub(1)..lines.len().min(y + 2) {
                    for j in x.saturating_sub(1)..lines[0].len().min(x + digits.len() + 1) {
                        if !lines[i][j].is_ascii_digit() && lines[i][j] != '.' {
                            sum += digits.iter().fold(0, |acc, dig| acc * 10 + dig);
                            break;
                        }
                    }
                }
                x += digits.len() - 1;
            }
            x += 1;
        }
    }
    sum
}

#[aoc(test = "467835")]
fn part_2(inp: &str) -> u32 {
    let mut sum = 0;
    let lines: Vec<Vec<char>> = inp.lines().map(|line| line.chars().collect()).collect();

    // Make a mask of the input, with each coordinate representing the number it is a part of, else 0.
    let mut number_mask = Vec::with_capacity(lines.len());
    for y in 0..lines.len() {
        let mut x = 0;
        let mut line = Vec::new();
        while x < lines[0].len() {
            if lines[y][x].is_ascii_digit() {
                let mut digits = vec![lines[y][x].to_digit(10).unwrap()];
                for i in x + 1..lines[0].len() {
                    if lines[y][i].is_ascii_digit() {
                        digits.push(lines[y][i].to_digit(10).unwrap())
                    } else {
                        break;
                    }
                }
                let number = digits.iter().fold(0, |acc, dig| acc * 10 + dig);
                (0..digits.len()).for_each(|_| line.push(number));
                x += digits.len() - 1;
            } else {
                line.push(0);
            }
            x += 1;
        }
        number_mask.push(line);
    }
    for y in 0..lines.len() {
        for x in 0..lines[0].len() {
            if lines[y][x] == '*' {
                let mut numbers = HashSet::new();
                for i in y.saturating_sub(1)..lines.len().min(y + 2) {
                    for j in x.saturating_sub(1)..lines[0].len().min(x + 2) {
                        if number_mask[i][j] > 0 {
                            numbers.insert(number_mask[i][j]);
                        }
                    }
                }
                if numbers.len() == 2 {
                    sum += numbers.iter().product::<u32>();
                }
            }
        }
    }
    sum
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
