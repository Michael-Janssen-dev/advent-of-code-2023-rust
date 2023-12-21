use std::collections::HashSet;

use aoc_macros::aoc;

// The test set needs 6 steps.
// #[aoc(test = "16")]
fn part_1(inp: &str) -> usize {
    let mut path: Vec<Vec<char>> = inp.lines().map(|line| line.chars().collect()).collect();
    let start = path
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter().enumerate().find_map(|(j, c)| {
                if *c == 'S' {
                    Some((i as isize, j as isize))
                } else {
                    None
                }
            })
        })
        .unwrap();
    path[start.0 as usize][start.1 as usize] = '.';
    calc(&path, start, 64)
}

fn calc(path: &[Vec<char>], start: (isize, isize), steps: u64) -> usize {
    let y_len = path.len() as isize;
    let x_len = path[0].len() as isize;
    let mut points = HashSet::new();
    points.insert(start);
    for _i in 1..=steps {
        let mut new_points = HashSet::new();
        for (y, x) in points {
            for (dy, dx) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                if path[((y + dy).rem_euclid(y_len)) as usize]
                    [((x + dx).rem_euclid(x_len)) as usize]
                    == '.'
                {
                    new_points.insert((y + dy, x + dx));
                }
            }
        }
        points = new_points
    }
    points.len()
}

#[aoc(test = "16")]
fn part_2(inp: &str) -> usize {
    let mut path: Vec<Vec<char>> = inp.lines().map(|line| line.chars().collect()).collect();
    let start = path
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter().enumerate().find_map(|(j, c)| {
                if *c == 'S' {
                    Some((i as isize, j as isize))
                } else {
                    None
                }
            })
        })
        .unwrap();
    path[start.0 as usize][start.1 as usize] = '.';

    // A bit of unnecessary calculation, but more functional this way.
    let y_0 = calc(&path, start, 65);
    let y_1 = calc(&path, start, 65 + 131);
    let y_2 = calc(&path, start, 65 + 131 * 2);
    let steps = 26501365;
    let x: usize = steps / 131;
    (y_2 - 2 * y_1 + y_0) * x.pow(2) / 2 + (2 * y_1 - 3 * y_0 / 2 - y_2 / 2) * x + y_0
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
