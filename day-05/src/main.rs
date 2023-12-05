use aoc_macros::aoc;

type SeedRange = (u64, u64);

struct Range {
    src_start: u64,
    dest_start: u64,
    len: u64
}

impl Range {
    fn map(&self, src: u64) -> Option<u64> {
        if self.src_start <= src && src < self.src_start + self.len {
            Some(self.dest_start + (src - self.src_start))
        } else {
            None
        }
    }
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let mut values = value.splitn(3, " ");
        Range {
            dest_start: values.next().unwrap().parse().unwrap(),
            src_start: values.next().unwrap().parse().unwrap(),
            len: values.next().unwrap().parse().unwrap()
        }
    }
}

struct Map {
    ranges: Vec<Range>
}

impl Map {
    fn map(&self, src: u64) -> u64 {
        self.ranges.iter().find_map(|r| r.map(src)).unwrap_or(src)
    }
    fn map_range(&self, src: Vec<SeedRange>) -> Vec<SeedRange> {
        let mut new_mapping = Vec::new();
        for (src, len) in src.iter() {
            let mut cur_src = *src;
            let mut cur_len = *len;
            for range in &self.ranges {
                if range.src_start <= cur_src && cur_src < range.src_start + range.len {
                    if cur_src + cur_len <= range.src_start + range.len {
                        new_mapping.push((range.dest_start + (cur_src - range.src_start), cur_len));
                        cur_len = 0;
                        break
                    } else {
                        new_mapping.push((range.dest_start + (cur_src - range.src_start), range.src_start + range.len - cur_src));
                        let new_src = range.src_start + range.len;
                        cur_len = cur_len - (new_src - cur_src);
                        cur_src = new_src;
                    }
                }
            }
            if cur_len > 0 {
                new_mapping.push((cur_src, cur_len));
            }
        }
        new_mapping
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut ranges: Vec<Range> = value.split("\n").skip(1).map(Range::from).collect();
        ranges.sort_by_key(|range| range.src_start);
        Map {
            ranges
        }
    }
}

#[aoc(test="35")]
fn part_1(inp: &str) -> u64 {
    let (seeds, mut maps) = inp.split_once("\n\n").unwrap();
    let mut seeds: Vec<u64> = seeds.split(" ").filter_map(|x| x.parse().ok()).collect();
    while let Some((map, new_maps)) = maps.split_once("\n\n") {
        let map = Map::from(map);
        seeds = seeds.iter().map(|s| map.map(*s)).collect();
        maps = new_maps;
    }
    let map = Map::from(maps);
    seeds = seeds.iter().map(|s| map.map(*s)).collect();
    *seeds.iter().min().unwrap()
}

#[aoc(test="46")]
fn part_2(inp: &str) -> u64 {
    let (seed_str, mut maps) = inp.split_once("\n\n").unwrap();
    let mut seeds = Vec::new();
    let mut seed_iter = seed_str.split(" ").skip(1);
    while let (Some(a), Some(b)) = (seed_iter.next(), seed_iter.next()) {
        seeds.push((a.parse().unwrap(), b.parse().unwrap()));
    }
    while let Some((map, new_maps)) = maps.split_once("\n\n") {
        let map = Map::from(map);
        seeds = map.map_range(seeds);
        maps = new_maps;
    }
    let map = Map::from(maps);
    seeds = map.map_range(seeds);
    seeds.iter().map(|&s| s.0).min().unwrap()
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
