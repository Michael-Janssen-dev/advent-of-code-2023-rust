use std::str::FromStr;

use aoc_macros::aoc;
use itertools::Itertools;
use z3::{
    ast::{self, Ast},
    Config, Context, Solver,
};

type Coor<T> = (T, T, T);
type P1Type = f64;
type P1Coor = Coor<P1Type>;

fn parse_line<T: FromStr + Copy>(line: &str) -> (Coor<T>, Coor<T>) {
    let (p, v) = line.split_once(" @ ").unwrap();
    let p: Vec<_> = p.split(',').filter_map(|x| x.trim().parse().ok()).collect();
    let v: Vec<_> = v.split(',').filter_map(|x| x.trim().parse().ok()).collect();
    ((p[0], p[1], p[2]), (v[0], v[1], v[2]))
}

fn get_intersect(l1: (P1Coor, P1Coor), l2: (P1Coor, P1Coor)) -> Option<(P1Type, P1Type)> {
    let ((px1, py1, _), (vx1, vy1, _)) = l1;
    let ((px2, py2, _), (vx2, vy2, _)) = l2;
    let s1 = vy1 / vx1;
    let s2 = vy2 / vx2;
    if s1 == s2 {
        return None;
    }
    let x = ((s2 * px2 - s1 * px1) - (py2 - py1)) / (s2 - s1);
    let y = (vy1 * (x - px1)) / vx1 + py1;
    let first_in_future = (x - px1).signum() == vx1.signum();
    let second_in_future = (x - px2).signum() == vx2.signum();
    if !first_in_future || !second_in_future {
        return None;
    }
    Some((x, y))
}
const LOWER_BOUND: f64 = 200000000000000.0;
const UPPER_BOUND: f64 = 400000000000000.0;

// const LOWER_BOUND: f64 = 7.0;
// const UPPER_BOUND: f64 = 27.0;
// #[aoc(test = "2")]
fn part_1(inp: &str) -> u64 {
    let lines: Vec<_> = inp.lines().map(parse_line).collect();
    lines
        .iter()
        .combinations(2)
        .filter(|v| {
            let l1 = v[0];
            let l2 = v[1];
            let point = get_intersect(*l1, *l2);
            if point.is_none() {
                return false;
            }
            let (x, y) = point.unwrap();
            (LOWER_BOUND..=UPPER_BOUND).contains(&x) && (LOWER_BOUND..=UPPER_BOUND).contains(&y)
        })
        .count()
        .try_into()
        .unwrap()
}

#[aoc(test = "47")]
fn part_2(inp: &str) -> i64 {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let opt = Solver::new(&ctx);

    let lines: Vec<_> = inp.lines().map(parse_line).collect();
    let x = ast::Int::new_const(&ctx, "x");
    let y = ast::Int::new_const(&ctx, "y");
    let z = ast::Int::new_const(&ctx, "z");
    let vx = ast::Int::new_const(&ctx, "vx");
    let vy = ast::Int::new_const(&ctx, "vy");
    let vz = ast::Int::new_const(&ctx, "vz");

    let mut i = 0;
    for ((px, py, pz), (pvx, pvy, pvz)) in lines.iter().take(6) {
        let t = ast::Int::new_const(&ctx, format!("t{i}"));
        let px = ast::Int::from_i64(&ctx, *px);
        let py = ast::Int::from_i64(&ctx, *py);
        let pz = ast::Int::from_i64(&ctx, *pz);
        let pvx = ast::Int::from_i64(&ctx, *pvx);
        let pvy = ast::Int::from_i64(&ctx, *pvy);
        let pvz = ast::Int::from_i64(&ctx, *pvz);
        opt.assert(&(&vx * &t + &x)._eq(&(&pvx * &t + &px)));
        opt.assert(&(&vy * &t + &y)._eq(&(&pvy * &t + &py)));
        opt.assert(&(&vz * &t + &z)._eq(&(&pvz * &t + &pz)));
        i += 1;
    }
    opt.check();
    let m = opt.get_model().unwrap();
    let x = m.eval(&x, true).unwrap().as_i64().unwrap();
    let y = m.eval(&y, true).unwrap().as_i64().unwrap();
    let z = m.eval(&z, true).unwrap().as_i64().unwrap();
    x + y + z
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
