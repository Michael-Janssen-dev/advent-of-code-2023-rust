use aoc_macros::aoc;

struct Race {
    dist: u64,
    time: u64,
}

#[aoc(test = "288")]
fn part_1(inp: &str) -> u64 {
    let mut lines = inp.lines();
    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    let distances: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    let races: Vec<Race> = times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &dist)| Race { dist, time })
        .collect();
    let mut possibilities = Vec::new();
    for race in &races {
        let mut start = 0;
        for i in 0..race.time {
            if (race.time - i) * i > race.dist {
                start = i;
                break;
            }
        }
        let mut end = 0;
        for i in (0..=race.time).rev() {
            if (race.time - i) * i > race.dist {
                end = i;
                break;
            }
        }
        possibilities.push(end - start + 1);
    }
    possibilities.iter().product()
}

#[aoc(test = "71503")]
fn part_2(inp: &str) -> u64 {
    let mut lines = inp.lines();
    let time: u64 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .unwrap();
    let dist: u64 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .unwrap();
    let mut start = 0;
    for i in 0..time {
        if (time - i) * i > dist {
            start = i;
            break;
        }
    }
    let mut end = 0;
    for i in (0..=time).rev() {
        if (time - i) * i > dist {
            end = i;
            break;
        }
    }
    end - start + 1
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
