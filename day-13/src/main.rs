use aoc_macros::aoc;

#[derive(Debug)]
struct Mirror {
    verticals: Vec<u32>,
    horizontals: Vec<u32>,
}

impl From<&str> for Mirror {
    fn from(value: &str) -> Self {
        let lines: Vec<_> = value
            .lines()
            .map(|l| l.replace('.', "0").replace('#', "1"))
            .collect();
        let horizontals = lines
            .iter()
            .map(|l| u32::from_str_radix(l, 2).unwrap())
            .collect();
        let mut verticals = (0..lines[0].len())
            .map(|_| String::new())
            .collect::<Vec<_>>();
        for row in lines {
            for (item, transposed_row) in row.chars().zip(&mut verticals) {
                transposed_row.push(item);
            }
        }
        let verticals = verticals
            .iter()
            .map(|l| u32::from_str_radix(l, 2).unwrap())
            .collect();
        Self {
            horizontals,
            verticals,
        }
    }
}

fn find_palindrome(items: &[u32]) -> Option<usize> {
    let equal_x =
        items
            .windows(2)
            .enumerate()
            .filter_map(|(i, s)| if s[0] == s[1] { Some(i) } else { None });
    for x in equal_x {
        let mut i = x;
        let mut j = x + 1;
        loop {
            if i == 0 || j == items.len() - 1 {
                return Some(x);
            }
            i -= 1;
            j += 1;
            if items[i] != items[j] {
                break;
            }
        }
    }
    None
}

// I could'nt find anywhere in the task what you should do if two reflection lines are found,
// but it works if the largest reflection is returned.
fn find_smudgy_palindrome(items: &[u32]) -> Option<usize> {
    let mut xs = vec![];
    for x in 0..items.len() - 1 {
        let mut i = x;
        let mut j = x + 1;
        let mut smudge = false;
        loop {
            if items[i] != items[j] {
                let diff = items[i].abs_diff(items[j]);
                if !smudge && ((diff & (diff - 1)) == 0) {
                    smudge = true;
                } else {
                    break;
                }
            }
            if i == 0 || j == items.len() - 1 {
                if !smudge {
                    break;
                }
                xs.push((j - i, x));
                break;
            }
            i -= 1;
            j += 1;
        }
    }
    if xs.is_empty() {
        return None;
    }
    xs.sort();
    Some(xs[xs.len() - 1].1)
}

#[aoc(test = "405")]
fn part_1(inp: &str) -> u32 {
    let mirrors: Vec<_> = inp.split("\n\n").map(Mirror::from).collect();
    let mut sum = 0;
    for mirror in &mirrors {
        let horizontal = find_palindrome(&mirror.horizontals);
        if let Some(x) = horizontal {
            sum += (x + 1) * 100;
            continue;
        }
        let vertical = find_palindrome(&mirror.verticals).unwrap();
        sum += vertical + 1;
    }
    sum as u32
}

#[aoc(test = "400")]
fn part_2(inp: &str) -> u32 {
    let mirrors: Vec<_> = inp.split("\n\n").map(Mirror::from).collect();
    let mut sum = 0;
    for (_i, mirror) in mirrors.iter().enumerate() {
        let horizontal = find_smudgy_palindrome(&mirror.horizontals);
        if let Some(x) = horizontal {
            sum += (x + 1) * 100;
            continue;
        }
        let vertical = find_smudgy_palindrome(&mirror.verticals).unwrap();
        sum += vertical + 1;
    }
    sum as u32
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
