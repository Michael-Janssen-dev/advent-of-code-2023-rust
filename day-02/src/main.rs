use aoc_macros::aoc;

#[derive(Default, Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

fn parse_game(line: &str) -> Game {
    let (id, rest) = line.split_once(": ").unwrap();
    let id = id.split_once(' ').unwrap().1.parse().unwrap();
    let mut rounds = vec![];
    for round_str in rest.split("; ") {
        let mut round = Round::default();
        for cubes in round_str.split(", ") {
            let (n, color) = cubes.split_once(' ').unwrap();
            let n = n.parse().unwrap();
            match color {
                "red" => round.red = n,
                "green" => round.green = n,
                "blue" => round.blue = n,
                _ => unreachable!("Parse error!"),
            }
        }
        rounds.push(round);
    }
    Game { id, rounds }
}

#[aoc(test = "8")]
fn part_1(inp: &str) -> u32 {
    let games: Vec<Game> = inp.lines().map(parse_game).collect();
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
    let games: Vec<Game> = inp.lines().map(parse_game).collect();
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
