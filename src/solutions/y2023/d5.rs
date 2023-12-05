use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

use crate::solutions::Solver;

use crate::solutions::AocError;

pub enum Solution {}

impl Solution {
    fn solve_range(m: &Map, start: u64, end: u64) -> Vec<(u64, u64)> {
        for d in m.data.iter() {
            if end < d.1 {
                return vec![(start, end)];
            } else if start < d.1 {
                let mut res = Solution::solve_range(m, d.1, end);
                res.push((start, d.1));
                return res;
            } else if end < d.1 + d.2 {
                let ds = d.0 + start - d.1;
                let de = d.0 + end - d.1;
                return vec![(ds, de)];
            } else if start < d.1 + d.2 {
                let mut res = Solution::solve_range(m, d.1 + d.2, end);
                let ds = d.0 + start - d.1;
                let de = d.0 + d.2;
                res.push((ds, de));
                return res;
            }
        }
        return vec![(start, end)];
    }
}

pub struct Map {
    source: String,
    dest: String,
    data: Vec<(u64, u64, u64)>,
}

pub struct Data {
    seeds: Vec<u64>,
    maps: HashMap<String, Map>,
}

impl Solver for Solution {
    type Input = Data;
    type Output = u64;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        let mut data = Data {
            seeds: vec![],
            maps: HashMap::new(),
        };

        let regex = Regex::new(r"(.*)-to-(.*) map:").unwrap();
        let mut cmap: Option<Map> = None;
        for l in input.lines() {
            if l.starts_with("seeds:") {
                data.seeds = l
                    .strip_prefix("seeds: ")
                    .unwrap()
                    .split(' ')
                    .map(|e| e.parse().unwrap())
                    .collect_vec();
            } else if let Some(c) = regex.captures(l) {
                if let Some(mut m) = cmap.take() {
                    m.data.sort_by_key(|k| k.1);
                    data.maps.insert(m.source.clone(), m);
                }

                let (_, caps) = c.extract::<2>();
                let source = caps[0].into();
                let dest = caps[1].into();

                let map = Map {
                    source,
                    dest,
                    data: vec![],
                };
                cmap = Some(map);
            } else if l.is_empty() {
                if let Some(mut m) = cmap.take() {
                    m.data.sort_by_key(|k| k.1);
                    data.maps.insert(m.source.clone(), m);
                }
            } else {
                let line = l
                    .split(' ')
                    .map(|e| e.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                if let Some(m) = cmap.as_mut() {
                    m.data.push(line);
                }
            }
        }
        if let Some(mut m) = cmap.take() {
            m.data.sort_by_key(|k| k.1);
            data.maps.insert(m.source.clone(), m);
        }

        Ok(data)
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, AocError> {
        let mut loc = u64::MAX;
        for s in input.seeds {
            let mut nextdest = "seed";
            let mut value = s;
            while let Some(m) = input.maps.get(nextdest) {
                print!("{} ({}), ", nextdest, value);
                nextdest = &m.dest;
                for d in m.data.iter() {
                    if value < d.1 {
                        break;
                    } else if value < d.1 + d.2 {
                        value = d.0 + (value - d.1);
                        break;
                    }
                }
            }
            println!("{} ({})", nextdest, value);

            if value < loc {
                loc = value;
            }
        }

        Ok(loc)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, AocError> {
        let mut ranges = vec![];
        for s in input.seeds.chunks(2) {
            let start = s[0];
            let end = start + s[1];
            ranges.push((start, end))
        }

        let mut nextdest = "seed";
        let mut new_ranges = vec![];
        while let Some(m) = input.maps.get(nextdest) {
            println!("{}-{}", m.source, m.dest);
            nextdest = &m.dest;

            for r in ranges {
                let mut sq = Solution::solve_range(&m, r.0, r.1);
                new_ranges.append(&mut sq);
            }
            ranges = new_ranges;
            new_ranges = vec![];
        }

        let res = ranges.iter().map(|a| a.0).min().unwrap();

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::Solver;

    use super::Solution;

    #[test]
    fn d5() {
        let i = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let r = Solution::solve(i, false).unwrap();
        dbg!(r);
    }
}
