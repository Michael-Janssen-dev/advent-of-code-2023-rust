use std::collections::{HashSet, VecDeque};

use aoc_macros::aoc;

#[derive(PartialEq, Eq, Hash, Clone)]
struct State(usize, usize, Direction);

enum Tile {
    Empty,
    HorizontalSplit,
    VerticalSplit,
    RightMirror,
    LeftMirror,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '-' => Tile::HorizontalSplit,
            '|' => Tile::VerticalSplit,
            '/' => Tile::RightMirror,
            '\\' => Tile::LeftMirror,
            _ => Tile::Empty,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Map(Vec<Vec<Tile>>);

impl Map {
    fn left(&self, state: &State) -> Option<State> {
        if 0 < state.1 {
            Some(State(state.0, state.1 - 1, Left))
        } else {
            None
        }
    }

    fn right(&self, state: &State) -> Option<State> {
        if state.1 < self.0[0].len() - 1 {
            Some(State(state.0, state.1 + 1, Right))
        } else {
            None
        }
    }

    fn up(&self, state: &State) -> Option<State> {
        if 0 < state.0 {
            Some(State(state.0 - 1, state.1, Up))
        } else {
            None
        }
    }

    fn down(&self, state: &State) -> Option<State> {
        if state.0 < self.0[0].len() - 1 {
            Some(State(state.0 + 1, state.1, Down))
        } else {
            None
        }
    }

    fn interact(&self, state: &State) -> Vec<State> {
        let tile = &self.0[state.0][state.1];
        let new_states = match tile {
            Tile::Empty => match state.2 {
                Left => vec![self.left(state)],
                Right => vec![self.right(state)],
                Up => vec![self.up(state)],
                Down => vec![self.down(state)],
            },
            Tile::HorizontalSplit => match state.2 {
                Left => vec![self.left(state)],
                Right => vec![self.right(state)],
                Up => vec![self.left(state), self.right(state)],
                Down => vec![self.left(state), self.right(state)],
            },
            Tile::VerticalSplit => match state.2 {
                Left => vec![self.up(state), self.down(state)],
                Right => vec![self.up(state), self.down(state)],
                Up => vec![self.up(state)],
                Down => vec![self.down(state)],
            },
            Tile::RightMirror => match state.2 {
                Left => vec![self.down(state)],
                Right => vec![self.up(state)],
                Up => vec![self.right(state)],
                Down => vec![self.left(state)],
            },
            Tile::LeftMirror => match state.2 {
                Left => vec![self.up(state)],
                Right => vec![self.down(state)],
                Up => vec![self.left(state)],
                Down => vec![self.right(state)],
            },
        };

        new_states.into_iter().flatten().collect()
    }

    fn simulate_beam(&self, start: State) -> u32 {
        let mut seen: HashSet<State> = HashSet::new();
        let mut queue: VecDeque<State> = VecDeque::new();
        queue.push_back(start);
        while let Some(state) = queue.pop_front() {
            if seen.contains(&state) {
                continue;
            }
            seen.insert(state.clone());
            for new_state in self.interact(&state) {
                if !seen.contains(&new_state) {
                    queue.push_back(new_state);
                }
            }
        }
        seen.iter()
            .map(|State(y, x, _)| (*y, *x))
            .collect::<HashSet<(usize, usize)>>()
            .len() as u32
    }
}

use Direction::*;

#[aoc(test = "46")]
fn part_1(inp: &str) -> u32 {
    let map: Vec<Vec<_>> = inp
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect();
    let map = Map(map);
    map.simulate_beam(State(0, 0, Right))
}

#[aoc(test = "51")]
fn part_2(inp: &str) -> u32 {
    let map: Vec<Vec<_>> = inp
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect();
    let map = Map(map);
    let mut states = Vec::new();
    for i in 0..map.0[0].len() {
        states.push(State(0, i, Down));
        states.push(State(map.0.len() - 1, i, Up));
    }
    for i in 0..map.0.len() {
        states.push(State(i, 0, Right));
        states.push(State(i, map.0.len() - 1, Left));
    }
    states
        .into_iter()
        .map(|s| map.simulate_beam(s))
        .max()
        .unwrap()
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
