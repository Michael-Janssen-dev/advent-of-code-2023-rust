use std::collections::{HashMap, VecDeque, HashSet};

use aoc_macros::aoc;
use rand::Rng;

fn build_adj_matrix(inp: &str) -> (HashMap<&str, usize>, Vec<Vec<isize>>) {
    let mut cur = 0;
    let mut map = HashMap::new();
    for line in inp.lines() {
        let (start, end) = line.split_once(": ").unwrap();
        map.entry(start).or_insert_with(|| {
            cur += 1;
            cur - 1
        });
        end.split(' ').for_each(|item| {
            map.entry(item).or_insert_with(|| {
                cur += 1;
                cur - 1
            });
        });
    }
    let mut adj = vec![vec![0; map.len()]; map.len()];

    for line in inp.lines() {
        let (start, end) = line.split_once(": ").unwrap();
        let start = map[start];
        let end: Vec<_> = end.split(' ').map(|x| map[x]).collect();
        end.iter().for_each(|&e| {
            adj[start][e] = 1;
            adj[e][start] = 1;
    })
    }
    (map, adj)
}

fn contract(mut edges: Vec<(usize, usize, usize)>, n: usize) -> (usize, Vec<usize>) {
    let mut rng = rand::thread_rng();
    for _i in 1..(n - 1) {
        let i = rng.gen_range(0..edges.len());
        let (s, t, _ind) = edges[i];
        edges.remove(i);
        for edge in &mut edges {
            if edge.0 == t {
                *edge = (edge.1.min(s), s.max(edge.1), edge.2);
            } else if edge.1 == t {
                *edge = (edge.0.min(s), s.max(edge.0), edge.2);
            }
        }
        edges.retain(|e| e.0 != e.1);
    }
    (edges.len(), edges.iter().map(|e| e.2).collect())
}

fn build_edges(adj: &Vec<Vec<isize>>) -> Vec<(usize, usize, usize)> {
    let mut edges = Vec::new();
    let mut c = 0;
    for i in 0..adj.len(){
        for j in i..adj.len() {
            if adj[i][j] == 1 {
                edges.push((i, j, c));
                c += 1;
            }
        }
    }
    edges
}

fn get_components(mut adj: Vec<Vec<isize>>, cut: Vec<(usize, usize)>, n: usize) -> Vec<HashSet<usize>> {
    for (a, b) in cut {
        adj[a][b] = 0;
        adj[b][a] = 0;
    }
    let mut components = Vec::new();
    let mut visited = HashSet::new();
    loop {
        if visited.len() == n {
            return components;
        }
        let mut component = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((0..n).find(|x| !visited.contains(x)).unwrap());
        while let Some(a) = queue.pop_front() {
            if visited.contains(&a) {
                continue;
            }
            visited.insert(a);
            component.insert(a);
            for x in 0..n {
                if adj[a][x] == 1 && !visited.contains(&x) {
                    queue.push_back(x);
                }
            }
        }
        components.push(component);
    }
}

fn find_cut(edges: Vec<(usize, usize, usize)>, n: usize) -> Vec<usize> {
    loop {
        let cut = contract(edges.clone(), n);
        if cut.0 == 3 {
            return cut.1
        }
    }
}

#[aoc(test = "54")]
fn part_1(inp: &str) -> usize {
    let (_map, adj) = build_adj_matrix(inp);
    let n = adj.len();
    let edges = build_edges(&adj);
    let cut = find_cut(edges.clone(), n);
    let cut = cut.iter().map(|m| (edges[*m].0, edges[*m].1)).collect();
    get_components(adj.clone(), cut, n).iter().map(|x| x.len()).product()
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
}
