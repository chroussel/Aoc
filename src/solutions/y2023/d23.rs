use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use crate::solutions::Solver;
use itertools::Itertools;

use crate::solutions::common::Map;
use crate::solutions::AocError;

pub enum Solution {}

impl Solution {}

impl Solver for Solution {
    type Input = Map<char>;
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        Ok(Map(input
            .lines()
            .map(|l| l.chars().collect_vec())
            .collect_vec()))
    }

    fn solve_part1(map: Self::Input) -> Result<Self::Output, AocError> {
        let (x, y) = map.shape();

        let mut start = (0i32, 0i32);
        let mut end = (0i32, 0i32);
        for j in 0..y {
            if map.0[0][j] == '.' {
                start = (0, j as i32);
            }
            if map.0[y - 1][j] == '.' {
                end = (y as i32 - 1, j as i32);
            }
        }

        let mut queue = VecDeque::new();
        queue.push_back((start, HashSet::new()));

        let mut res = vec![];
        while let Some((q, mut path)) = queue.pop_front() {
            if !path.insert(q) {
                continue;
            }
            if q == end {
                res.push(path.len() - 1);
            }
            let m = map.0[q.0 as usize][q.1 as usize];
            let neigh = match m {
                '>' => vec![(0, 1)],
                '<' => vec![(0, -1)],
                '^' => vec![(-1, 0)],
                'v' => vec![(1, 0)],
                '.' => vec![(0, 1), (0, -1), (1, 0), (-1, 0)],
                _ => vec![],
            };
            for n in neigh {
                let newpos = (q.0 + n.0, q.1 + n.1);
                if newpos.0 < 0 || newpos.1 < 0 || newpos.0 as usize >= x || newpos.1 as usize >= y
                {
                    continue;
                }
                let m = map.0[newpos.0 as usize][newpos.1 as usize];
                if m != '#' {
                    queue.push_back((newpos, path.clone()));
                }
            }
        }
        println!("{:?}", res);

        let r = *res.iter().max().unwrap();
        Ok(r as i32)
    }

    fn solve_part2(map: Self::Input) -> Result<Self::Output, AocError> {
        let (x, y) = map.shape();

        let mut start = (0i32, 0i32);
        let mut end = (0i32, 0i32);
        for j in 0..y {
            if map.0[0][j] == '.' {
                start = (0, j as i32);
            }
            if map.0[y - 1][j] == '.' {
                end = (y as i32 - 1, j as i32);
            }
        }

        let mut edges = Vec::new();

        let mut queue = VecDeque::new();
        queue.push_back((start, start, -1));
        let mut visited = HashSet::new();
        while let Some((prev, p, d)) = queue.pop_front() {
            if !visited.insert((prev, p)) {
                continue;
            }
            //println!("{:?} {:?}", prev, p);
            let neigh = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
            let mut paths = vec![];
            let mut sum_path = (0, 0);
            for n in neigh {
                let newpos = (p.0 + n.0, p.1 + n.1);
                if newpos.0 < 0 || newpos.1 < 0 || newpos.0 as usize >= x || newpos.1 as usize >= y
                {
                    continue;
                }
                let m = map.0[newpos.0 as usize][newpos.1 as usize];
                if m != '#' {
                    paths.push(newpos);
                    sum_path.0 += n.0;
                    sum_path.1 += n.1;
                }
            }

            if p == end {
                edges.push((end, prev, d + 1));
                edges.push((prev, end, d + 1));
            }
            println!("{:?} {:?} {:?}", prev, p, sum_path);
            if paths.len() > 2 {
                for n in paths.iter() {
                    queue.push_back((p, n.clone(), 0));
                    if &prev != n {
                        edges.push((p, prev, d + 1));
                        edges.push((prev, p, d + 1));
                    }
                }
            } else {
                for n in paths {
                    queue.push_back((prev, n, d + 1));
                }
            }
        }
        let mut intersections = HashMap::new();
        let mut dists = HashMap::new();
        for (k, v, d) in edges {
            intersections.entry(k).or_insert(HashSet::new()).insert(v);
            intersections.entry(v).or_insert(HashSet::new()).insert(k);
            dists.insert((k, v), d);
            dists.insert((v, k), d);
        }

        let mut queue = VecDeque::new();
        //let mut cache = HashMap::new();
        queue.push_back((start, BTreeSet::new(), 0));
        let mut maxpath = 0;
        while let Some((q, mut path, dist)) = queue.pop_front() {
            if q == end {
                maxpath = maxpath.max(dist);
                //println!("{:?}", dist);
                continue;
            }
            /*if let Some(d) = cache.get(&path) {
                if *d > dist {
                    continue;
                }
            }*/
            //cache.insert(path.clone(), dist);
            if let Some(ne) = intersections.get(&q) {
                for n in ne {
                    let d = dists.get(&(q, n.clone())).unwrap();
                    let mut newpath = path.clone();
                    if newpath.insert(n.clone()) {
                        //if let Some(v) = cache.get(&newpath) {
                        //    if *v < dist + d {
                        queue.push_back((n.clone(), newpath, dist + d))
                        //   }
                        //} else {
                        //  queue.push_back((n.clone(), newpath, dist + d))
                        //}
                    }
                }
            }
        }

        Ok(maxpath as i32)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::Solver;

    use super::Solution;

    #[test]
    fn d23() {
        let i = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";

        dbg!(Solution::solve(i, true));
        dbg!(Solution::solve(i, false));
    }
}
