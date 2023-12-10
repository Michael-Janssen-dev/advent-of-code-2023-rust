use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

use aoc_macros::aoc;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Pipe {
    NS,
    NE,
    NW,
    SE,
    SW,
    EW,
    S,
    E,
}
use Pipe::*;

impl Pipe {
    fn is_south(&self) -> bool {
        *self == NS || *self == SE || *self == SW
    }
    fn is_north(&self) -> bool {
        *self == NS || *self == NE || *self == NW
    }
    fn is_east(&self) -> bool {
        *self == EW || *self == SE || *self == NE
    }
    fn is_west(&self) -> bool {
        *self == EW || *self == SW || *self == NW
    }
    fn is_horizontally_connected(&self, other: &Pipe) -> bool {
        self.is_east() && other.is_west()
    }
    fn is_vertically_connected(&self, other: &Pipe) -> bool {
        self.is_south() && other.is_north()
    }
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::NS,
            'L' => Self::NE,
            'J' => Self::NW,
            'F' => Self::SE,
            '7' => Self::SW,
            '-' => Self::EW,
            'S' => Self::S,
            _ => Self::E,
        }
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            NS => "|",
            NE => "L",
            NW => "J",
            SE => "F",
            SW => "7",
            EW => "-",
            S => "S",
            E => ".",
        })
    }
}

type Coor = (usize, usize);
type Map = Vec<Vec<Vec<Coor>>>;

fn parse_pipes(inp: &str) -> Vec<Vec<Pipe>> {
    inp.lines()
        .map(|line| line.chars().map(Pipe::from).collect())
        .collect()
}

fn get_map(pipes: &mut [Vec<Pipe>]) -> (Map, Coor) {
    let mut map: Map = Vec::new();
    let x_n = pipes[0].len();
    let y_n = pipes.len();
    let mut s = (0, 0);
    for (y, line) in pipes.iter().enumerate() {
        let mut pipe_line = Vec::new();
        for (x, &p) in line.iter().enumerate() {
            if p == S {
                s = (y, x);
            }
            let mut neighbours = Vec::new();
            if y > 0 && (p == NS || p == NW || p == NE) {
                neighbours.push((y - 1, x));
            }
            if y < y_n && (p == NS || p == SW || p == SE) {
                neighbours.push((y + 1, x));
            }
            if x > 0 && (p == EW || p == SW || p == NW) {
                neighbours.push((y, x - 1));
            }
            if x < x_n && (p == EW || p == SE || p == NE) {
                neighbours.push((y, x + 1));
            }
            pipe_line.push(neighbours);
        }
        map.push(pipe_line);
    }
    let mut north = false;
    if s.0 > 0 && map[s.0 - 1][s.1].contains(&s) {
        map[s.0][s.1].push((s.0 - 1, s.1));
        north = true;
    }
    let mut south = false;
    if s.0 < map.len() - 1 && map[s.0 + 1][s.1].contains(&s) {
        map[s.0][s.1].push((s.0 + 1, s.1));
        south = true;
    }
    let mut west = false;
    if s.1 > 0 && map[s.0][s.1 - 1].contains(&s) {
        map[s.0][s.1].push((s.0, s.1 - 1));
        west = true;
    }
    let mut east = false;
    if s.1 < map[0].len() && map[s.0][s.1 + 1].contains(&s) {
        map[s.0][s.1].push((s.0, s.1 + 1));
        east = true;
    }
    pipes[s.0][s.1] = match (north, south, east, west) {
        (true, true, false, false) => NS,
        (true, false, true, false) => NE,
        (true, false, false, true) => NW,
        (false, true, true, false) => SE,
        (false, true, false, true) => SW,
        (false, false, true, true) => EW,
        _ => unreachable!("Start is not connected to two pipes"),
    };
    (map, s)
}

#[aoc(test = "8", part = 1)]
fn part_1(inp: &str) -> u32 {
    let mut pipes = parse_pipes(inp);
    let (map, s) = get_map(&mut pipes);
    let mut last = s;
    let mut c = map[s.0][s.1][0];
    let mut len = 1;
    loop {
        let prev_last = last;
        last = c;
        let neighbours = &map[c.0][c.1];
        if neighbours[0] != prev_last {
            c = neighbours[0];
        } else {
            c = neighbours[1];
        }
        if c == s {
            break;
        }
        len += 1;
    }
    len / 2 + 1
}

fn expand_pipes(pipes: &[Vec<Pipe>]) -> Vec<Vec<Pipe>> {
    let space_pipes: Vec<Vec<Pipe>> = pipes
        .iter()
        .map(|line| {
            let mut pipe_line = Vec::new();
            line.windows(2).for_each(|v| {
                let (i, j) = (v[0], v[1]);
                pipe_line.push(i);
                if i.is_horizontally_connected(&j) {
                    pipe_line.push(EW);
                } else {
                    pipe_line.push(E);
                }
            });
            pipe_line.push(line[line.len() - 1]);
            pipe_line
        })
        .collect();
    let mut vertical_pipes = vec![vec![E; space_pipes[0].len()]; space_pipes.len() * 2 - 1];
    for x in 0..space_pipes[0].len() {
        for y in 0..space_pipes.len() - 1 {
            let i = space_pipes[y][x];
            let j = space_pipes[y + 1][x];
            vertical_pipes[2 * y][x] = i;
            if i.is_vertically_connected(&j) {
                vertical_pipes[2 * y + 1][x] = NS;
            } else {
                vertical_pipes[2 * y + 1][x] = E;
            }
        }
        vertical_pipes[2 * (space_pipes.len() - 1)][x] = space_pipes[space_pipes.len() - 1][x]
    }
    vertical_pipes
}

fn filter_loop_pipe(pipes: &[Vec<Pipe>], mask: &[Vec<bool>]) -> Vec<Vec<Pipe>> {
    let mut filtered = Vec::new();
    for i in 0..pipes.len() {
        let mut line = Vec::new();
        for j in 0..pipes[0].len() {
            if mask[i][j] {
                line.push(pipes[i][j]);
            } else {
                line.push(E);
            }
        }
        filtered.push(line);
    }
    filtered
}

fn get_loop_mask(map: &Map, start: Coor) -> Vec<Vec<bool>> {
    let mut mask = vec![vec![false; map[0].len()]; map.len()];
    mask[start.0][start.1] = true;
    let mut last = start;
    let mut c = map[start.0][start.1][0];
    loop {
        mask[c.0][c.1] = true;
        let prev_last = last;
        last = c;
        let neighbours = &map[c.0][c.1];
        if neighbours[0] != prev_last {
            c = neighbours[0];
        } else {
            c = neighbours[1];
        }
        if c == start {
            break;
        }
    }
    mask
}

fn flood_fill(mask: &mut [Vec<bool>], pipes: &[Vec<Pipe>]) {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    for x in 0..pipes[0].len() {
        if pipes[0][x] == E {
            queue.push_back((0, x))
        }
        if pipes[pipes.len() - 1][x] == E {
            queue.push_back((pipes.len() - 1, x));
        }
    }
    for y in 0..pipes.len() {
        if pipes[y][0] == E {
            queue.push_back((y, 0))
        }
        if pipes[y][pipes[0].len() - 1] == E {
            queue.push_back((y, pipes[0].len() - 1));
        }
    }
    while !queue.is_empty() {
        let (y, x) = queue.pop_front().unwrap();
        if visited.contains(&(y, x)) {
            continue;
        }
        visited.insert((y, x));
        if x % 2 == 0 && y % 2 == 0 {
            mask[y / 2][x / 2] = true;
        }
        if y > 0 && pipes[y - 1][x] == E && !visited.contains(&(y - 1, x)) {
            queue.push_back((y - 1, x));
        }
        if x > 0 && pipes[y][x - 1] == E && !visited.contains(&(y, x - 1)) {
            queue.push_back((y, x - 1));
        }
        if y < pipes.len() - 1
            && pipes[y + 1][x] == E
            && !visited.contains(&(y + 1, x))
        {
            queue.push_back((y + 1, x));
        }
        if x < pipes[0].len() - 1
            && pipes[y][x + 1] == E
            && !visited.contains(&(y, x + 1))
        {
            queue.push_back((y, x + 1));
        }
    }
}

#[aoc(test = "10", part = 2)]
fn part_2(inp: &str) -> u32 {
    let mut pipes = parse_pipes(inp);
    let (map, s) = get_map(&mut pipes);
    let mut mask = get_loop_mask(&map, s);

    // Filter out all pieces of pipe not connected to the loop
    let filtered_pipes = filter_loop_pipe(&pipes, &mask);

    // "Zoom in" on the pipes, filling in the space between the pieces
    let expanded_pipes = expand_pipes(&filtered_pipes);

    // Flood fill from the edges, filing in all spaces not in the loop
    flood_fill(&mut mask, &expanded_pipes);
    mask.iter()
        .flat_map(|line| line.iter().filter(|line| !**line))
        .count() as u32
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
