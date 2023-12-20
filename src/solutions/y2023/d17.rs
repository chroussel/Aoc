use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::solutions::Solver;

use crate::solutions::common::Map;
use crate::solutions::AocError;

pub enum Solution {}

impl Solution {}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    fn to_p(&self) -> (i32, i32) {
        match self {
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
        }
    }
    fn oppo(&self) -> Dir {
        match self {
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
        }
    }
}

impl Solver for Solution {
    type Input = Map<u32>;
    type Output = u32;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        Ok(Map(input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
            .collect_vec()))
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, AocError> {
        let mut v = VecDeque::new();
        v.push_front(((0, 0), Dir::Right, 0));

        let (x, y) = input.shape();

        let mut dist: HashMap<((i32, i32), Dir, u32), u32> = HashMap::new();
        dist.insert(((0, 0), Dir::Right, 0), 0);
        let target = (x as i32 - 1, y as i32 - 1);
        while let Some((p, cur_dir, samedircounter)) = v.pop_front() {
            let score = *dist.get(&(p, cur_dir, samedircounter)).unwrap();
            let neigh = if samedircounter == 2 {
                if cur_dir == Dir::Right || cur_dir == Dir::Left {
                    vec![Dir::Up, Dir::Down]
                } else {
                    vec![Dir::Right, Dir::Left]
                }
            } else {
                vec![Dir::Up, Dir::Down, Dir::Right, Dir::Left]
            };
            for n in neigh {
                if n == cur_dir.oppo() {
                    continue;
                }
                let next_dir_conter = if cur_dir == n { samedircounter + 1 } else { 0 };
                let asp = n.to_p();
                let newp = (p.0 + asp.0, p.1 + asp.1);
                if newp.0 < 0 || newp.0 >= x as i32 || newp.1 < 0 || newp.1 >= y as i32 {
                    continue;
                }
                let cell = input.0[newp.0 as usize][newp.1 as usize];
                let newdist = score + cell;

                if let Some(&prevd) = dist.get(&(newp, n, next_dir_conter)) {
                    if prevd > newdist {
                        dist.insert((newp, n, next_dir_conter), newdist);
                        v.push_back((newp, n, next_dir_conter));
                    }
                } else {
                    dist.insert((newp, n, next_dir_conter), newdist);
                    v.push_back((newp, n, next_dir_conter));
                }
            }
        }

        let mut min_dist = u32::MAX;
        for d in vec![Dir::Up, Dir::Down, Dir::Right, Dir::Left] {
            for i in 0..3 {
                if let Some(d) = dist.get(&(target, d, i)) {
                    min_dist = min_dist.min(*d);
                }
            }
        }

        Ok(min_dist)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, AocError> {
        let mut v = VecDeque::new();
        v.push_front(((0, 0), Dir::Right, 0, 0));

        let (x, y) = input.shape();

        let mut dist: HashMap<((i32, i32), Dir, u32, u32), u32> = HashMap::new();
        let mut prev = HashMap::new();

        dist.insert(((0, 0), Dir::Right, 0, 0), 0);
        let target = (x as i32 - 1, y as i32 - 1);
        //println!("{:?}", target);
        while let Some((p, cur_dir, samedircounter, wobble)) = v.pop_front() {
            let score = *dist.get(&(p, cur_dir, samedircounter, wobble)).unwrap();
            let neigh = if samedircounter < 3 {
                vec![cur_dir]
            } else if wobble == 9 {
                if cur_dir == Dir::Right || cur_dir == Dir::Left {
                    vec![Dir::Up, Dir::Down]
                } else {
                    vec![Dir::Right, Dir::Left]
                }
            } else {
                vec![Dir::Up, Dir::Down, Dir::Right, Dir::Left]
            };
            /*println!(
                "{:?} {:?} {} {} {:?} {}",
                p, cur_dir, samedircounter, wobble, neigh, score
            );*/
            for n in neigh {
                if n == cur_dir.oppo() {
                    continue;
                }
                let mut next_dir_conter = 0;
                let mut wobble_counter = 0;
                if cur_dir == n {
                    next_dir_conter = samedircounter + 1;
                    wobble_counter = wobble + 1;
                };
                let asp = n.to_p();
                let newp = (p.0 + asp.0, p.1 + asp.1);
                if newp.0 < 0 || newp.0 >= x as i32 || newp.1 < 0 || newp.1 >= y as i32 {
                    continue;
                }
                let cell = input.0[newp.0 as usize][newp.1 as usize];
                let newdist = score + cell;

                if let Some(&prevd) = dist.get(&(newp, n, next_dir_conter, wobble_counter)) {
                    if prevd > newdist {
                        dist.insert((newp, n, next_dir_conter, wobble_counter), newdist);
                        v.push_back((newp, n, next_dir_conter, wobble_counter));
                        prev.insert(
                            (newp, n, next_dir_conter, wobble_counter),
                            (p, cur_dir, samedircounter, wobble),
                        );
                    }
                } else {
                    dist.insert((newp, n, next_dir_conter, wobble_counter), newdist);
                    v.push_back((newp, n, next_dir_conter, wobble_counter));
                    prev.insert(
                        (newp, n, next_dir_conter, wobble_counter),
                        (p, cur_dir, samedircounter, wobble),
                    );
                }
            }
        }

        let mut min_dist = u32::MAX;
        let mut minpath = vec![];
        for (p, _) in prev.iter() {
            if p.0 == target {
                if p.2 < 3 {
                    continue;
                }
                let mut path = vec![];
                let mut next = p.clone();
                while let Some(a) = prev.get(&next) {
                    path.push(a.0);
                    next = a.clone();
                }
                path.reverse();
                let score = dist.get(p).unwrap();
                //println!("{:?}: {}", path, score);
                if *score < min_dist {
                    min_dist = *score;
                    minpath = path;
                }
            }
        }
        println!("minPath: {:?}", minpath);
        Ok(min_dist)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::Solver;

    use super::Solution;

    #[test]
    fn d17() {
        let i = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

        let i2 = "111111111111
999999999991
999999999991
999999999991
999999999991";

        dbg!(Solution::solve(i, false));
        dbg!(Solution::solve(i2, false));
    }
}
