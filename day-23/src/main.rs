use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use aoc_macros::aoc;

const D_TO_DIR: [((isize, isize), char); 4] =
    [((0, 1), '>'), ((0, -1), '<'), ((1, 0), 'v'), ((-1, 0), '^')];

type Vertex = (isize, isize);
type Edge = (usize, usize, isize);

fn build_graph(map: &[Vec<char>], cyclic: bool) -> (Vec<Vertex>, Vec<Edge>) {
    let start: (isize, isize) = (0, map[0].iter().position(|c| *c == '.').unwrap() as isize);
    let end: (isize, isize) = (
        (map.len() - 1) as isize,
        map[map.len() - 1].iter().position(|c| *c == '.').unwrap() as isize,
    );
    let mut queue = VecDeque::new();
    queue.push_back(((start.0 + 1, start.1), (1, 0), 0, 1));
    let mut v_n = 2;
    let mut edges = vec![];
    let mut vertices = vec![];
    vertices.push(start);
    vertices.push(end);
    while let Some(((y, x), (py, px), mut v, mut dist)) = queue.pop_front() {
        if (y, x) == end {
            edges.push((v, 1, dist));
            continue;
        }
        let mut next = Vec::new();
        for (dy, dx) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if py == -dy && px == -dx {
                //skip the previous
                continue;
            }
            if cyclic {
                if map[(y + dy) as usize][(x + dx) as usize] != '#' {
                    next.push((dy, dx));
                }
                continue;
            }
            if map[(y + dy) as usize][(x + dx) as usize] == '.' {
                next.push((dy, dx));
            }
            if D_TO_DIR
                .iter()
                .find_map(|(d, c)| if *d == (dy, dx) { Some(*c) } else { None })
                .unwrap()
                == map[(y + dy) as usize][(x + dx) as usize]
            {
                next.push((dy, dx));
            }
        }
        if next.len() > 1 {
            let v_i = vertices.iter().position(|v| *v == (y, x)).unwrap_or(v_n);
            edges.push((v, v_i, dist));
            if cyclic {
                edges.push((v_i, v, dist));
            }
            if v_i != v_n {
                continue;
            }
            vertices.push((y, x));
            v = v_n;
            v_n += 1;
            dist = 0;
        };
        for (dy, dx) in next {
            queue.push_back(((y + dy, x + dx), (dy, dx), v, dist + 1))
        }
    }
    (vertices, edges)
}

fn longest_path(edges: &[Edge], end: usize) -> usize {
    let mut queue = BinaryHeap::new();
    queue.push((0_isize, 0));
    let mut paths = HashSet::new();
    while let Some((dist, v)) = queue.pop() {
        if v == end {
            paths.insert(dist.abs() as usize);
        }
        let adjacent: Vec<_> = edges.iter().filter(|(a, _, _)| *a == v).collect();
        adjacent
            .iter()
            .for_each(|(_, v, d)| queue.push((dist as isize - d, *v)))
    }
    *paths.iter().max().unwrap()
}

fn longest_cyclic_path(
    node: usize,
    end: usize,
    edges: &HashMap<usize, HashMap<usize, isize>>,
    visited: u64,
) -> usize {
    if node == end {
        return 0;
    }
    let res = edges
        .get(&node)
        .unwrap()
        .iter()
        .filter_map(|(v, d)| {
            if visited & (1 << *v) == 0 {
                let new_cost = longest_cyclic_path(*v, end, &edges, visited ^ (1 << *v));
                Some(new_cost + *d as usize)
            } else {
                None
            }
        })
        .max()
        .unwrap_or(0);
    res
}
#[aoc(test = "94")]
fn part_1(inp: &str) -> usize {
    let map: Vec<Vec<_>> = inp.lines().map(|line| line.chars().collect()).collect();
    let (_, edges) = build_graph(&map, false);
    longest_path(&edges, 1)
}

fn longest_cyclic_path_2(node: (isize, isize), end: (isize, isize), map: &[Vec<char>], visited: HashSet<(isize, isize)>) -> usize {
    if node == end {
        return 0
    }
    let mut current_max = 0;
    for (dy, dx) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let (ny, nx) = (node.0 + dy, node.1 + dx);
        if map[ny as usize][nx as usize] == '#' {
            continue;
        }
        if visited.contains(&(node.0 + dy, node.1 + dx)) {
            continue;
        }
        let mut new_visited = visited.clone();
        new_visited.insert((ny, nx));
        let new_cost = longest_cyclic_path_2((ny, nx), end, map, new_visited);
        current_max = current_max.max(new_cost + 1)
    }
    current_max
}


#[aoc(test = "154")]
fn part_2(inp: &str) -> usize {
    let map: Vec<Vec<_>> = inp.lines().map(|line| line.chars().collect()).collect();
    let (vertices, edges) = build_graph(&map, true);
    let mut new_edges = HashMap::new();
    for (a, b, cost) in edges {
        let adj = new_edges.entry(a).or_insert(HashMap::new());
        let entry = adj.entry(b).or_insert(0);
        *entry = (*entry).max(cost);
    }
    let mut visited = 0;
    visited = visited ^ (1 << 0);
    eprintln!("{:?}", new_edges);
    // longest_cyclic_path(0, 1, &new_edges, visited)
    let mut visited = HashSet::new();
    visited.insert((0, 1));
    longest_cyclic_path_2((1, 1), ((map.len() - 1) as isize, (map[0].len() - 2) as isize), &map,visited ) + 1
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
