use aoc_macros::aoc;

#[derive(Default, Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl From<&str> for Round {
    fn from(value: &str) -> Self {
        let mut round = Round::default();
        for cubes in value.split(", ") {
            let (n, color) = cubes.split_once(' ').unwrap();
            let n = n.parse().unwrap();
            match color {
                "red" => round.red = n,
                "green" => round.green = n,
                "blue" => round.blue = n,
                _ => unreachable!("Parse error!"),
            }
        }
        round
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl From<&str> for Game {
    fn from(line: &str) -> Self {
        let (id, rest) = line.split_once(": ").unwrap();
        let id = id.split_once(' ').unwrap().1.parse().unwrap();
        let rounds = rest.split("; ").map(Round::from).collect();
        Game { id, rounds }
    }
}

#[aoc(test = "8")]
fn part_1(inp: &str) -> u32 {
    let games: Vec<Game> = inp.lines().map(Game::from).collect();
    games
        .iter()
        .filter_map(|g| {
            if g.rounds
                .iter()
                .all(|r| r.red <= 12 && r.green <= 13 && r.blue <= 14)
            {
                Some(g.id)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(test = "2286")]
fn part_2(inp: &str) -> u32 {
    let games: Vec<Game> = inp.lines().map(Game::from).collect();
    games
        .iter()
        .map(|g| {
            g.rounds.iter().fold((0, 0, 0), |(r, g, b), round| {
                (r.max(round.red), g.max(round.green), b.max(round.blue))
            })
        })
        .map(|(r, g, b)| r * g * b)
        .sum()
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
