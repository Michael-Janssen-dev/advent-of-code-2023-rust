use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use aoc_macros::aoc;

const D_TO_DIR: [((isize, isize), char); 4] =
    [((0, 1), '>'), ((0, -1), '<'), ((1, 0), 'v'), ((-1, 0), '^')];

type Vertex = (isize, isize);
type Edge = (usize, usize, isize);

fn build_graph(map: &[Vec<char>]) -> (Vec<Vertex>, Vec<Edge>) {
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

type Graph = HashMap<(isize, isize), HashSet<((isize, isize), isize)>>;

fn build_cyclic_graph(
    map: &[Vec<char>],
) -> Graph {
    let mut graph = HashMap::new();
    for y in 0..map.len() as isize {
        for x in 0..map[0].len() as isize {
            if map[y as usize][x as usize] == '#' {
                continue;
            }
            let mut new_vertex = HashSet::new();
            for (dy, dx) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let ny = y + dy;
                let nx = x + dx;
                if ny < 0 || nx < 0 || ny >= map.len() as isize || nx >= map[0].len() as isize {
                    continue;
                }
                if map[ny as usize][nx as usize] != '#' {
                    new_vertex.insert(((ny, nx), 1));
                }
            }
            graph.insert((y, x), new_vertex);
        }
    }
    let mut update = false;
    while !update {
        update = true;
        for key in graph.keys().copied().collect::<Vec<_>>() {
            if graph[&key].len() != 2 {
                continue;
            }

            let mut iter = graph[&key].iter();
            let (a, a_dist) = *iter.next().unwrap();
            let (b, b_dist) = *iter.next().unwrap();

            let a_point = graph.get_mut(&a).unwrap();
            a_point.retain(|(pos, _)| *pos != key);
            a_point.insert((b, a_dist + b_dist));

            let b_point = graph.get_mut(&b).unwrap();
            b_point.retain(|(pos, _)| *pos != key);
            b_point.insert((a, a_dist + b_dist));

            graph.remove(&key);

            update = false
        }
    }
    graph
}

fn longest_path(edges: &[Edge], end: usize) -> usize {
    let mut queue = BinaryHeap::new();
    queue.push((0_isize, 0));
    let mut paths = HashSet::new();
    while let Some((dist, v)) = queue.pop() {
        if v == end {
            paths.insert(dist.unsigned_abs());
        }
        let adjacent: Vec<_> = edges.iter().filter(|(a, _, _)| *a == v).collect();
        adjacent
            .iter()
            .for_each(|(_, v, d)| queue.push((dist - d, *v)))
    }
    *paths.iter().max().unwrap()
}
#[aoc(test = "94")]
fn part_1(inp: &str) -> usize {
    let map: Vec<Vec<_>> = inp.lines().map(|line| line.chars().collect()).collect();
    let (_, edges) = build_graph(&map);
    longest_path(&edges, 1)
}

#[aoc(test = "154")]
fn part_2(inp: &str) -> usize {
    let map: Vec<Vec<_>> = inp.lines().map(|line| line.chars().collect()).collect();
    let graph = build_cyclic_graph(&map);
    let mut queue = Vec::new();
    let mut visited = HashSet::new();
    let mut max = 0;

    let end: (isize, isize) = ((map.len() - 1) as isize, (map[0].len() - 2) as isize);

    queue.push(((0_isize, 1_isize), Some(0)));
    while let Some((pos, distance)) = queue.pop() {
        let Some(distance) = distance else {
            visited.remove(&pos);
            continue;
        };
        if pos == end {
            max = max.max(distance);
            continue;
        }
        if !visited.insert(pos) {
            continue;
        }
        queue.push((pos, None));
        for (pos, dist) in &graph[&pos] {
            queue.push((*pos, Some(distance + dist)));
        }
    }

    max as usize
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
