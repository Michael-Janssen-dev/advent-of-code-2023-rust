use std::{cmp::Ordering, collections::HashMap};

use aoc_macros::aoc;

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<(char, bool)> for Card {
    fn from(value: (char, bool)) -> Self {
        match value.0 {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => {
                if value.1 {
                    Self::Joker
                } else {
                    Self::Jack
                }
            }
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => {
                unreachable!("Wrong card")
            }
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Ord, Eq, Debug)]
struct Hand {
    cards: Vec<Card>,
    jokers: bool,
}

impl From<(&str, bool)> for Hand {
    fn from(value: (&str, bool)) -> Self {
        let cards = value.0.chars().map(|c| Card::from((c, value.1))).collect();
        Self {
            cards,
            jokers: value.1,
        }
    }
}

impl Hand {
    fn get_hand_type(&self) -> HandType {
        let mut count = HashMap::new();
        for card in &self.cards {
            *count.entry(card).or_insert(0) += 1;
        }
        let mut jokers = 0;
        if self.jokers {
            jokers = *count.get(&Card::Joker).unwrap_or(&0);
        }
        let max = *count
            .iter()
            .filter(|item| **item.0 != Card::Joker)
            .map(|item| item.1)
            .max()
            .unwrap_or(&0);
        if max + jokers == 5 {
            HandType::FiveOfAKind
        } else if max + jokers == 4 {
            HandType::FourOfAKind
        } else if max == 3 && count.values().any(|v| *v == 2) {
            HandType::FullHouse
        } else if max + jokers == 3 && count.values().filter(|v| **v == 2).count() == 2 {
            HandType::FullHouse
        } else if max + jokers == 3 {
            HandType::ThreeOfAKind
        } else if max == 2 && jokers == 1 {
            HandType::TwoPair
        } else if max == 2 && count.values().filter(|&v| *v == 2).count() == 2 {
            HandType::TwoPair
        } else if max + jokers == 2 {
            HandType::Pair
        } else if jokers == 1 {
            HandType::Pair
        } else {
            HandType::HighCard
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let hand_type = &self.get_hand_type();
        let other_hand_type = &other.get_hand_type();
        match hand_type.partial_cmp(other_hand_type) {
            Some(Ordering::Equal) => self.cards.partial_cmp(&other.cards),
            Some(x) => Some(x),
            None => None,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Bid {
    hand: Hand,
    bid: u32,
}

impl From<(&str, bool)> for Bid {
    fn from(value: (&str, bool)) -> Self {
        let (hand, bid) = value.0.split_once(' ').unwrap();
        let hand = Hand::from((hand, value.1));
        let bid = bid.parse().unwrap();
        Self { hand, bid }
    }
}

#[aoc(test = "6440")]
fn part_1(inp: &str) -> u32 {
    let mut bids: Vec<Bid> = inp.lines().map(|line| Bid::from((line, false))).collect();
    bids.sort();
    bids.iter()
        .enumerate()
        .map(|(i, bid)| bid.bid * (i + 1) as u32)
        .sum()
}

#[aoc(test = "5905")]
fn part_2(inp: &str) -> u32 {
    let mut bids: Vec<Bid> = inp.lines().map(|line| Bid::from((line, true))).collect();
    bids.sort();
    bids.iter()
        .enumerate()
        .map(|(i, bid)| bid.bid * (i + 1) as u32)
        .sum()
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
