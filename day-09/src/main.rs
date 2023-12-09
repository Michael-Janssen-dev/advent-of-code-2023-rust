use aoc_macros::aoc;

fn get_sequences(s: Vec<i32>) -> Vec<Vec<i32>> {
    let mut s = s;
    let mut diff = vec![s.clone()];
    loop {
        let mut new_sequence = Vec::new();
        for i in 0..s.len() - 1 {
            new_sequence.push(s[i + 1] - s[i])
        }
        if new_sequence.iter().all(|x| *x == 0) {
            break;
        };
        s = new_sequence.clone();
        diff.push(new_sequence);
    }
    diff
}

#[aoc(test = "114")]
fn part_1(inp: &str) -> i32 {
    let sequences = inp.lines().map(|line| {
        line.split_whitespace()
            .filter_map(|v| v.parse().ok())
            .collect::<Vec<i32>>()
    });
    let mut sum = 0;
    for s in sequences {
        let diff = get_sequences(s);
        let mut x = 0;
        for s in diff.iter().rev() {
            x += s.iter().last().unwrap();
        }
        sum += x;
    }
    sum
}

#[aoc(test = "2")]
fn part_2(inp: &str) -> i32 {
    let sequences = inp.lines().map(|line| {
        line.split_whitespace()
            .filter_map(|v| v.parse().ok())
            .collect::<Vec<i32>>()
    });
    let mut sum = 0;
    for s in sequences {
        let diff = get_sequences(s);
        let mut x = 0;
        for s in diff.iter().rev() {
            x = s.iter().next().unwrap() - x;
        }
        sum += x;
    }
    sum
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
