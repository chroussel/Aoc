use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::solutions::Solver;

use crate::solutions::common::Map;
use crate::solutions::AocError;

pub enum Solution {}

impl Solution {}

impl Solver for Solution {
    type Input = ((i64, i64), Map<char>);
    type Output = i64;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        let mut start = (0, 0);
        let res = input
            .lines()
            .enumerate()
            .map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .map(|(j, c)| {
                        if c == 'S' {
                            start = (i as i64, j as i64);
                            '.'
                        } else {
                            c
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();
        Ok((start, Map(res)))
    }

    fn solve_part1((start, map): Self::Input) -> Result<Self::Output, AocError> {
        let mut h = HashSet::new();
        h.insert(start);
        let mut next_c = HashSet::new();
        let neigh = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let (x, y) = map.shape();
        println!("{} {}", x, y);
        for _ in 0..64 {
            for p in h {
                //println!("{:?}", p);
                for n in neigh.iter() {
                    let newp = (p.0 + n.0, p.1 + n.1);
                    if newp.0 < 0 || newp.1 < 0 || newp.0 >= y as i64 || newp.1 >= x as i64 {
                        continue;
                    }
                    let elem = map.0[newp.0 as usize][newp.1 as usize];
                    match elem {
                        '.' => {
                            next_c.insert(newp.clone());
                        }
                        _ => {}
                    }
                }
            }
            h = next_c;
            next_c = HashSet::new();
        }

        Ok(h.len() as i64)
    }

    fn solve_part2((start, map): Self::Input) -> Result<Self::Output, AocError> {
        let (x, y) = map.shape();

        println!("{} {}", x, y);

        let cycle = 26501365;
        let s1 = cycle % x;
        let s2 = s1 + x;
        let s3 = s1 + x + x;

        let c1 = cycle_count(&map, start, s1);
        let c2 = cycle_count(&map, start, s2);
        let c3 = cycle_count(&map, start, s3);

        println!("{} {} {}", s1, s2, s3);
        println!("{} {} {}", c1, c2, c3);
        let s1f = s1 as f64;
        let s2f = s2 as f64;
        let s3f = s3 as f64;

        let n = nalgebra::matrix![
            1., s1f, s1f * s1f;
            1., s2f, s2f * s2f;
            1., s3f, s3f * s3f
        ];
        let cf = cycle as f64;

        let invn = n.try_inverse().unwrap();
        let ys = nalgebra::vector![c1 as f64, c2 as f64, c3 as f64];
        let scalars = invn * ys;

        let res2 = scalars[2] * (cf * cf) + scalars[1] * cf + scalars[0];
        println!("{}", res2);
        let c = c1;
        let b = (4 * c2 - 3 * c1 - c3) / 2;
        let a = (c3 + c1 - 2 * c2) / 2;
        let cx = (cycle / x) as i64;
        let res3 = a * cx * cx + b * cx + c;
        println!("{}", res3);

        Ok(res2.round() as i64)
    }
}

fn cycle_count(map: &Map<char>, start: (i64, i64), cycles: usize) -> i64 {
    let mut s = HashSet::new();
    s.insert(start);
    for _ in 0..cycles {
        s = next_cycle(map, &s)
    }
    s.len() as i64
}

fn next_cycle(map: &Map<char>, input: &HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    let neigh = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let (x, y) = map.shape();
    let x = x as i64;
    let y = y as i64;
    let mut next_c = HashSet::new();
    for p in input {
        //println!("{:?}", p);
        for n in neigh.iter() {
            let mut newp = ((p.0 + n.0) % x, (p.1 + n.1) % (y));
            if newp.0 < 0 {
                newp.0 += x;
            }
            if newp.1 < 0 {
                newp.1 += y;
            }
            let elem = map.0[newp.0 as usize][newp.1 as usize];
            match elem {
                '.' => {
                    next_c.insert((p.0 + n.0, p.1 + n.1));
                }
                _ => {}
            }
        }
    }

    next_c
}

#[cfg(test)]
mod tests {
    use crate::solutions::Solver;

    use super::Solution;

    #[test]
    fn d21() {
        let i = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

        dbg!(Solution::solve(i, false));
    }
}
