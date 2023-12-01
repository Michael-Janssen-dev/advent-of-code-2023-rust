const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|x| {
            x.chars().find_map(|y| y.to_digit(10)).unwrap() * 10
                + x.chars().rev().find_map(|c| c.to_digit(10)).unwrap()
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|x| {
            let mut first = None;
            let mut last = None;
            let mut iter = x.chars();
            for i in 0..x.len() {
                let c = iter.next().unwrap();
                if c.is_ascii_digit() {
                    last = c.to_digit(10);
                    if first.is_none() {
                        first = c.to_digit(10);
                    }
                    continue;
                }
                for (n_i, &num) in NUMBERS.iter().enumerate() {
                    if i + num.len() <= x.len() && num == &x[i..i + num.len()] {
                        last = Some(n_i as u32);
                        if first.is_none() {
                            first = Some(n_i as u32);
                        }
                        break;
                    }
                }
            }
            first.unwrap() * 10 + last.unwrap()
        })
        .sum()
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use crate::part_1;
    use crate::part_2;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../test-1.txt")), 142);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../test-2.txt")), 281);
    }
}
