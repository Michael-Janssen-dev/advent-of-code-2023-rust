use aoc_macros::aoc;
use itertools::Itertools;

type Scale = u64;
type Coor = (Scale, Scale);

struct Map {
    galaxies: Vec<Coor>,
    empty_horizontals: Vec<Scale>,
    empty_verticals: Vec<Scale>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let lines: Vec<&str> = value.lines().collect();
        let mut empty_horizontals = Vec::new();
        let mut empty = vec![true; lines[0].len()];
        let mut galaxies = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            let mut line_empty = true;
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    empty[j] = false;
                    line_empty = false;
                    galaxies.push((i as Scale, j as Scale));
                }
            }
            if line_empty {
                empty_horizontals.push(i as Scale);
            }
        }
        let empty_verticals: Vec<Scale> = empty
            .iter()
            .enumerate()
            .filter(|(_, x)| **x)
            .map(|(i, _)| i as Scale)
            .collect();
        Self {
            galaxies,
            empty_horizontals,
            empty_verticals,
        }
    }
}

impl Map {
    fn calc_distance_sum(&self, scale: Scale) -> Scale {
        self.galaxies
            .iter()
            .tuple_combinations()
            .map(|(&a, &b)| {
                a.0.abs_diff(b.0)
                    + a.1.abs_diff(b.1)
                    + self
                        .empty_horizontals
                        .iter()
                        .filter(|&&h| (a.0 < h && h < b.0) || (b.0 < h && h < a.0))
                        .count() as Scale
                        * (scale - 1)
                    + self
                        .empty_verticals
                        .iter()
                        .filter(|&&h| (a.1 < h && h < b.1 || (b.1 < h && h < a.1)))
                        .count() as Scale
                        * (scale - 1)
            })
            .sum()
    }
}

#[aoc(test = "374")]
fn part_1(inp: &str) -> Scale {
    let galaxy_map = Map::from(inp);
    galaxy_map.calc_distance_sum(2)
}

// #[aoc(test = "8410")]
fn part_2(inp: &str) -> Scale {
    let galaxy_map = Map::from(inp);
    galaxy_map.calc_distance_sum(1_000_000)
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
