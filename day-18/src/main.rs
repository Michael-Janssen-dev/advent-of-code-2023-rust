use aoc_macros::aoc;

type Scale = i64;

// Using Pick's formula and shoelace formula
fn calculate_area(vertices: &[(Scale, Scale)]) -> Scale {
    let mut area: i64 = 0;
    let mut last = &(0, 0);
    let mut b = 0;
    for next in vertices.iter().skip(1) {
        area += (last.0 + next.0) * (last.1 - next.1);
        b += ((next.0 - last.0) + (next.1 - last.1)).abs();
        last = next;
    }
    area += (last.0) * (last.1);
    b += last.0 + last.1;
    (area / 2) + (b / 2) + 1
}

#[aoc(test = "62")]
fn part_1(inp: &str) -> Scale {
    let mut vertices = Vec::new();
    let (mut y, mut x) = (0, 0);
    for line in inp.lines() {
        vertices.push((y, x));
        let (dir, rest) = line.split_once(' ').unwrap();
        let (len, _) = rest.split_once(' ').unwrap();
        let len: Scale = len.parse().unwrap();
        (y, x) = match dir {
            "U" => (y - len, x),
            "R" => (y, x + len),
            "D" => (y + len, x),
            "L" => (y, x - len),
            _ => unimplemented!("Unknown direction"),
        };
    }
    calculate_area(&vertices)
}

#[aoc(test = "952408144115")]
fn part_2(inp: &str) -> i64 {
    let mut vertices = vec![(0, 0)];
    let (mut y, mut x) = (0_i64, 0_i64);
    for line in inp.lines() {
        vertices.push((y, x));
        let rest = line.split_whitespace().nth(2).unwrap();
        let len =
            i64::from_str_radix(&rest.chars().skip(2).take(5).collect::<String>(), 16).unwrap();
        let dir = rest.chars().nth(7).unwrap();
        (y, x) = match dir {
            '3' => (y - len, x),
            '0' => (y, x + len),
            '1' => (y + len, x),
            '2' => (y, x - len),
            _ => unimplemented!("Unknown direction"),
        };
    }
    calculate_area(&vertices)
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
