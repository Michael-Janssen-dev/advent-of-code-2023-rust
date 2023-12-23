use std::collections::HashSet;

use aoc_macros::aoc;

type Scale = usize;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
enum Direction {
    X,
    Y,
    Z,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Cube {
    x: Scale,
    y: Scale,
    z: Scale,
    dir: Direction,
    end: Scale,
}

impl From<&str> for Cube {
    fn from(value: &str) -> Self {
        let (start, end) = value.split_once('~').unwrap();
        let [x, y, z] = start
            .splitn(3, ',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let [ex, ey, ez] = end
            .splitn(3, ',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let (dir, end) = match (ex - x, ey - y, ez - z) {
            (_, 0, 0) => (Direction::X, ex),
            (0, _, 0) => (Direction::Y, ey),
            (0, 0, _) => (Direction::Z, ez),
            _ => unimplemented!(),
        };
        Self { x, y, z, dir, end }
    }
}

impl Cube {
    fn overlaps(&self, other: &Self) -> bool {
        if !match (&self.dir, &other.dir) {
            (Direction::X, Direction::X) => self.x <= other.end && other.x <= self.end,
            (Direction::X, _) => self.x <= other.x && self.end >= other.x,
            (_, Direction::X) => other.x <= self.x && other.end >= self.x,
            (_, _) => self.x == other.x,
        } {
            return false;
        }
        if !match (&self.dir, &other.dir) {
            (Direction::Y, Direction::Y) => self.y <= other.end && other.y <= self.end,
            (Direction::Y, _) => self.y <= other.y && self.end >= other.y,
            (_, Direction::Y) => other.y <= self.y && other.end >= self.y,
            (_, _) => self.y == other.y,
        } {
            return false;
        }
        if !match (&self.dir, &other.dir) {
            (Direction::Z, Direction::Z) => self.z <= other.end && other.z <= self.end,
            (Direction::Z, _) => self.z <= other.z && self.end >= other.z,
            (_, Direction::Z) => other.z <= self.z && other.end >= self.z,
            (_, _) => self.z == other.z,
        } {
            return false;
        }
        return true;
    }
}

fn drop(index: usize, cubes: &[Cube]) -> Cube {
    let mut cube = cubes.get(index).unwrap().clone();
    while cube.z > 1 {
        cube.z -= 1;
        if cube.dir == Direction::Z {
            cube.end -= 1;
        }
        if cubes
            .iter()
            .enumerate()
            .any(|(i, c)| i != index && cube.overlaps(c))
        {
            cube.z += 1;
            if cube.dir == Direction::Z {
                cube.end += 1;
            }
            return cube;
        }
    }
    cube
}

fn try_drop(cubes: &[Cube]) -> bool {
    for c in 0..cubes.len() {
        let cube = drop(c, cubes);
        if cube != cubes[c] {
            return false;
        }
    }
    true
}

fn how_many_drop(mut cubes: Vec<Cube>) -> usize {
    let mut count = 0;
    for c in 0..cubes.len() {
        let cube = drop(c, &cubes);
        if cube != cubes[c] {
            count += 1;
        }
        *cubes.get_mut(c).unwrap() = cube;
    }
    count
}

#[aoc(test = "5")]
fn part_1(inp: &str) -> usize {
    let mut cubes: Vec<_> = inp.lines().map(Cube::from).collect();
    cubes.sort_by_key(|cube| cube.z);
    (0..cubes.len()).for_each(|i| {
        let cube = drop(i, &cubes);
        *cubes.get_mut(i).unwrap() = cube;
    });
    let mut count = 0;
    (0..cubes.len()).for_each(|i| {
        if try_drop(
            &cubes
                .iter()
                .enumerate()
                .filter(|(j, _)| i != *j)
                .map(|(_, c)| c.clone())
                .collect::<Vec<_>>(),
        ) {
            count += 1;
        }
    });
    count
}

#[aoc(test = "7")]
fn part_2(inp: &str) -> usize {
    let mut cubes: Vec<_> = inp.lines().map(Cube::from).collect();
    cubes.sort_by_key(|cube| cube.z);
    (0..cubes.len()).for_each(|i| {
        let cube = drop(i, &cubes);
        *cubes.get_mut(i).unwrap() = cube;
    });
    let mut count = 0;
    (0..cubes.len()).for_each(|i| {
        count += how_many_drop(
            cubes
                .iter()
                .enumerate()
                .filter(|(j, _)| i != *j)
                .map(|(_, c)| c.clone())
                .collect::<Vec<_>>(),
        );
    });
    count
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
