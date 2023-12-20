use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

use crate::solutions::Solver;

use crate::solutions::AocError;

pub enum Solution {}

impl Solution {
    fn neigh(elem: Elem) -> Vec<(i32, i32)> {
        let south = (0, 1);
        let north = (0, -1);
        let east = (1, 0);
        let west = (-1, 0);
        let allowed_neigh = match elem {
            Elem::SE => {
                vec![south, east]
            }
            Elem::NW => vec![north, west],
            Elem::NE => vec![north, east],
            Elem::V => vec![south, north],
            Elem::SW => vec![south, west],
            Elem::H => vec![east, west],
            Elem::S => vec![south, north, east, west],
            _ => {
                vec![]
            }
        };
        allowed_neigh
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Elem {
    V,
    H,
    NE,
    NW,
    SW,
    SE,
    Empty,
    S,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Solver for Solution {
    type Input = (Pos, Vec<Vec<Elem>>);
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        let mut map = vec![];
        let mut pos = Pos { x: 0, y: 0 };
        for l in input.lines() {
            let li = l
                .chars()
                .enumerate()
                .map(|(i, c)| match c {
                    '|' => Elem::V,
                    '-' => Elem::H,
                    'L' => Elem::NE,
                    'J' => Elem::NW,
                    '7' => Elem::SW,
                    'F' => Elem::SE,
                    'S' => {
                        pos.x = i as i32;
                        pos.y = map.len() as i32;
                        Elem::S
                    }
                    _ => Elem::Empty,
                })
                .collect_vec();
            map.push(li);
        }
        Ok((pos, map))
    }

    fn solve_part1((start, map): Self::Input) -> Result<Self::Output, AocError> {
        let mut maxd = 0;
        let mut stack = vec![start];
        let mut dist = HashMap::new();
        dist.insert(start, 0);

        while let Some(pos) = stack.pop() {
            let dp = dist.get(&pos).copied().unwrap();

            let curelem = map[pos.y as usize][pos.x as usize];

            //println!("{} {}: {:?} {:?}", pos.x, pos.y, curelem, dp);
            let allowed_neigh = Solution::neigh(curelem);

            for (x, y) in allowed_neigh {
                let next = Pos {
                    y: pos.y + y,
                    x: pos.x + x,
                };
                if next.y < 0 || next.y as usize >= map.len() {
                    continue;
                }
                if next.x < 0 || next.x as usize >= map[next.y as usize].len() {
                    continue;
                }
                let next_elem = map[next.y as usize][next.x as usize];

                if !Solution::neigh(next_elem).contains(&(-x, -y)) {
                    continue;
                }
                //if parent == Some(next) {
                //    continue;
                //}
                let nextd = dist.get(&next);

                match nextd {
                    Some(n) => {
                        if dp + 1 < *n {
                            stack.push(next);
                            dist.insert(next, dp + 1);
                        }
                    }
                    None => {
                        dist.insert(next, dp + 1);
                        stack.push(next);
                    }
                }
            }
        }

        for (_, d) in dist {
            if d > maxd {
                maxd = d
            }
        }
        Ok(maxd)
    }

    fn solve_part2((start, map): Self::Input) -> Result<Self::Output, AocError> {
        let mut stack: Vec<(Option<Pos>, Pos)> = vec![(None, start)];
        let mut path = vec![];
        path.push(start);
        let mut loopElem = HashSet::new();
        let mut pred = HashMap::new();
        let mut next = HashMap::new();
        while let Some((prev, pos)) = stack.pop() {
            let curelem = map[pos.y as usize][pos.x as usize];
            //println!("{} {}: {:?}", pos.x, pos.y, curelem);

            if let Some(p) = prev {
                path.push(pos);
                pred.insert(p, pos);
                next.insert(pos, p);
                loopElem.insert(pos);
                loopElem.insert(p);
                if curelem == Elem::S {
                    break;
                }
            }

            let allowed_neigh = Solution::neigh(curelem);
            for (x, y) in allowed_neigh {
                let next = Pos {
                    y: pos.y + y,
                    x: pos.x + x,
                };
                if next.y < 0 || next.y as usize >= map.len() {
                    continue;
                }
                if next.x < 0 || next.x as usize >= map[next.y as usize].len() {
                    continue;
                }
                let next_elem = map[next.y as usize][next.x as usize];

                if next_elem == Elem::Empty {
                    continue;
                }
                if !Solution::neigh(next_elem).contains(&(-x, -y)) {
                    continue;
                }
                if Some(next) == prev {
                    continue;
                }
                stack.push((Some(pos), next));
            }
        }

        println!(
            "{}, {}, {}, {}",
            path.len(),
            pred.len(),
            loopElem.len(),
            next.len()
        );
        println!("DFS done");

        let mut em = vec![];
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                let p = Pos {
                    x: j as i32,
                    y: i as i32,
                };
                if !loopElem.contains(&p) {
                    em.push(p);
                }
            }
        }

        //shoelace formula
        let mut area: i32 = 0;
        let n = path.len() as i32;
        for w in path.windows(2) {
            area += (w[0].x * w[1].y) as i32;

            area -= (w[0].y * w[1].x) as i32;
        }
        let area = i32::abs(area) / 2;

        Ok(area - n / 2 + 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::Solver;

    use super::Solution;

    #[test]
    fn d10() {
        let i = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";
        dbg!(Solution::solve(i, true));
    }

    #[test]
    fn d10_2() {
        let i = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";
        let i2 = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

        let i3 = "..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........";

        let i4 = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        let i5 = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        assert_eq!("10", Solution::solve(i4, false).unwrap());
        assert_eq!("4", Solution::solve(i3, false).unwrap());
        assert_eq!("4", Solution::solve(i2, false).unwrap());

        assert_eq!("8", Solution::solve(i5, false).unwrap());
    }
}
