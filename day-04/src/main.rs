use std::collections::HashSet;

use aoc_macros::aoc;

struct Card {
    numbers: HashSet<u32>,
    winning: HashSet<u32>,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let (_, numbers) = value.split_once(": ").unwrap();
        let (card, winning) = numbers.split_once(" | ").unwrap();
        let numbers = card
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let winning = winning
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        Card { numbers, winning }
    }
}

impl Card {
    fn winning(&self) -> u32 {
        self.numbers.intersection(&self.winning).count() as u32
    }
}

#[aoc(test = "13")]
fn part_1(inp: &str) -> u32 {
    let cards: Vec<Card> = inp.lines().map(Card::from).collect();
    cards.iter().map(|c| 2_u32.pow(c.winning() / 2)).sum()
}

#[aoc(test = "30")]
fn part_2(inp: &str) -> u32 {
    let cards: Vec<Card> = inp.lines().map(Card::from).collect();
    let mut count = vec![1; cards.len()];
    for (i, c) in cards.iter().enumerate() {
        let copies = *count.get(i).unwrap();
        for j in 1..=c.winning() {
            if let Some(c) = count.get_mut(i + j as usize) {
                *c += copies;
            }
        }
    }
    count.iter().sum()
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
