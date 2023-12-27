use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

use crate::solutions::Solver;

use crate::solutions::common::Map;
use crate::solutions::AocError;

pub enum Solution {}

impl Solution {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos(i64, i64, i64);

impl Solver for Solution {
    type Input = Vec<(Pos, Pos)>;
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        Ok(input
            .lines()
            .map(|l| {
                let (a, b) = l.split_once('~').unwrap();
                let p1: (i64, i64, i64) = a
                    .split(',')
                    .map(|o| o.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                let p2: (i64, i64, i64) = b
                    .split(',')
                    .map(|o| o.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                (Pos(p1.0, p1.1, p1.2), Pos(p2.0, p2.1, p2.2))
            })
            .collect_vec())
    }

    fn solve_part1(mut input: Self::Input) -> Result<Self::Output, AocError> {
        let max = input.iter().fold((0, 0, 0), |a, (b, c)| {
            let x = (a.0.max(b.0).max(c.0));
            let y = (a.1.max(b.1).max(c.1));
            let z = (a.2.max(b.2).max(c.2));
            (x, y, z)
        });
        let maxA = (max.0 as usize + 1, max.1 as usize + 1, max.2 as usize + 1);

        println!("{:?}", max);

        input.sort_by_key(|a| a.0 .2.min(a.1 .2));
        let mut tree: HashMap<usize, Vec<usize>> = HashMap::new();
        let newpos = update(&input, maxA);
        for (idp, p) in newpos.iter().enumerate() {
            for (idq, q) in newpos.iter().enumerate() {
                if support(p, q) {
                    tree.entry(idp).or_insert(vec![]).push(idq);
                }
            }
        }

        let mut invtree = HashMap::new();

        for (k, v) in tree.iter() {
            for a in v {
                invtree.entry(*a).or_insert(vec![]).push(*k);
            }
        }
        println!("{:?}", tree);
        println!("{:?}", invtree);

        let mut count = 0;
        for id in 0..input.len() {
            if let Some(supports) = tree.get(&id) {
                let mut canremove = true;
                for a in supports {
                    if let Some(supported_by) = invtree.get(a) {
                        if supported_by.len() == 1 {
                            canremove = false;
                        }
                    } else {
                        unreachable!()
                    }
                }
                if canremove {
                    count += 1;
                }
            } else {
                count += 1;
            }
        }

        Ok(count)
    }

    fn solve_part2(mut input: Self::Input) -> Result<Self::Output, AocError> {
        let max = input.iter().fold((0, 0, 0), |a, (b, c)| {
            let x = (a.0.max(b.0).max(c.0));
            let y = (a.1.max(b.1).max(c.1));
            let z = (a.2.max(b.2).max(c.2));
            (x, y, z)
        });
        let maxA = (max.0 as usize + 1, max.1 as usize + 1, max.2 as usize + 1);

        println!("{:?}", max);

        input.sort_by_key(|a| a.0 .2.min(a.1 .2));
        let mut tree: HashMap<usize, Vec<usize>> = HashMap::new();
        let newpos = update(&input, maxA);
        for (idp, p) in newpos.iter().enumerate() {
            for (idq, q) in newpos.iter().enumerate() {
                if support(p, q) {
                    tree.entry(idp).or_insert(vec![]).push(idq);
                }
            }
        }

        let mut invtree = HashMap::new();

        for (k, v) in tree.iter() {
            for a in v {
                invtree.entry(*a).or_insert(vec![]).push(*k);
            }
        }

        let mut count = 0;
        for id in 0..input.len() {
            count += dfs(&tree, &invtree, id);
        }

        Ok(count as i32)
    }
}

fn dfs(
    tree: &HashMap<usize, Vec<usize>>,
    invtree: &HashMap<usize, Vec<usize>>,
    pos: usize,
) -> usize {
    let mut dead_nodes = HashSet::new();
    let mut queue = VecDeque::new();
    dead_nodes.insert(pos);
    if let Some(supports) = tree.get(&pos) {
        for s in supports {
            queue.push_back(*s);
        }
    }

    while let Some(n) = queue.pop_front() {
        if dead_nodes.contains(&n) {
            continue;
        }
        //println!("current: {}; {:?} {:?}", n, dead_nodes, queue);
        let sub_count = if let Some(supports) = invtree.get(&n) {
            supports.iter().filter(|s| !dead_nodes.contains(*s)).count()
        } else {
            0
        };

        if sub_count == 0 {
            dead_nodes.insert(n);

            if let Some(supports) = tree.get(&n) {
                for s in supports {
                    queue.push_back(*s);
                }
            }
        }
    }
    return dead_nodes.len() - 1;
}

fn support(a: &(Pos, Pos), b: &(Pos, Pos)) -> bool {
    let bz = b.0 .2.min(b.1 .2);
    let az = a.0 .2.max(a.1 .2);

    if bz == az + 1 {
        let r1 = Rect::from_pos(a);
        let r2 = Rect::from_pos(b);
        return r1.intersect_rect(&r2);
    } else {
        false
    }
}

struct Rect {
    left: i64,
    right: i64,
    top: i64,
    bottom: i64,
}
impl Rect {
    fn from_pos(a: &(Pos, Pos)) -> Self {
        Rect {
            left: a.0 .1.min(a.1 .1),
            right: a.0 .1.max(a.1 .1),
            top: a.0 .0.min(a.1 .0),
            bottom: a.0 .0.max(a.1 .0),
        }
    }
    fn intersect_rect(&self, r: &Rect) -> bool {
        return !(r.left > self.right
            || r.right < self.left
            || r.top > self.bottom
            || r.bottom < self.top);
    }
}

fn update(input: &Vec<(Pos, Pos)>, (x, y, _): (usize, usize, usize)) -> Vec<(Pos, Pos)> {
    let mut max_height = vec![vec![0; y]; x];
    let mut newpos = vec![];
    for (start, end) in input {
        let z = start.2.min(end.2);
        let z2 = start.2.max(end.2);

        let zlength = z2 - z + 1;
        //println!("{}", zlength);

        let mut maxz = 0;
        for i in start.0..(end.0 + 1) {
            for j in start.1..(end.1 + 1) {
                let curh = max_height[i as usize][j as usize];
                maxz = maxz.max(curh);
            }
        }
        if maxz < z {
            for i in start.0..(end.0 + 1) {
                for j in start.1..(end.1 + 1) {
                    max_height[i as usize][j as usize] = maxz + zlength;
                }
            }
        }
        newpos.push((
            Pos(start.0, start.1, maxz),
            Pos(end.0, end.1, maxz + zlength - 1),
        ));
        //println!("{:?}~{:?} {}", start, end, maxz);
        //println!("{:?}", maxHeight);
    }
    newpos
}

#[cfg(test)]
mod tests {
    use crate::solutions::Solver;

    use super::Solution;

    #[test]
    fn d22() {
        let i = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";

        dbg!(Solution::solve(i, true));
        dbg!(Solution::solve(i, false));
    }
}
