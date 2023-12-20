use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

use crate::solutions::Solver;

use crate::solutions::AocError;

pub enum Solution {}

impl Solution {}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Elem {
    Empty,
    Galaxy,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Solver for Solution {
    type Input = Vec<Vec<Elem>>;
    type Output = i64;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        Ok(input
            .lines()
            .map(|l| {
                l.chars()
                    .filter_map(|c| {
                        return match c {
                            '.' => Some(Elem::Empty),
                            '#' => Some(Elem::Galaxy),
                            _ => None,
                        };
                    })
                    .collect_vec()
            })
            .collect_vec())
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, AocError> {
        let mut r = HashSet::new();
        for i in 0..input.len() {
            let mut found = true;

            for j in 0..input[i].len() {
                if input[i][j] == Elem::Galaxy {
                    found = false;
                    break;
                }
            }
            if found {
                r.insert(i as i32);
            }
        }
        let mut c = HashSet::new();

        for j in 0..input[0].len() {
            let mut found = true;
            for i in 0..input.len() {
                if input[i][j] == Elem::Galaxy {
                    found = false;
                    break;
                }
            }
            if found {
                c.insert(j as i32);
            }
        }

        println!("rows: {:?}", r);
        println!("cols: {:?}", c);

        let mut galaxies = vec![];
        for i in 0..input[0].len() {
            for j in 0..input.len() {
                if input[i][j] == Elem::Galaxy {
                    galaxies.push(Pos {
                        x: j as i32,
                        y: i as i32,
                    })
                }
            }
        }

        let countcombi = (galaxies.len() - 1) * galaxies.len() / 2;

        println!("{} => {}", galaxies.len(), countcombi);

        let mut sum = 0;
        for i in 0..galaxies.len() {
            for j in (i + 1)..galaxies.len() {
                let g = galaxies[i];
                let h = galaxies[j];
                let minx = g.x.min(h.x);
                let maxx = g.x.max(h.x);
                let miny = g.y.min(h.y);
                let maxy = g.y.max(h.y);
                let dist = maxx - minx + maxy - miny;
                let count_rows = c.iter().filter(|&&ri| ri > minx && ri < maxx).count() as i32;
                let count_cols = r.iter().filter(|&&ci| ci > miny && ci < maxy).count() as i32;

                let fd = dist + count_rows + count_cols;
                /*
                println!(
                    "dist {:?} to {:?} => {} ({}, {}, {})",
                    g, h, fd, dist, count_rows, count_cols
                );*/
                sum += fd;
            }
        }
        Ok(sum.into())
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, AocError> {
        let mut r = HashSet::new();
        for i in 0..input.len() {
            let mut found = true;

            for j in 0..input[i].len() {
                if input[i][j] == Elem::Galaxy {
                    found = false;
                    break;
                }
            }
            if found {
                r.insert(i as i32);
            }
        }
        let mut c = HashSet::new();

        for j in 0..input[0].len() {
            let mut found = true;
            for i in 0..input.len() {
                if input[i][j] == Elem::Galaxy {
                    found = false;
                    break;
                }
            }
            if found {
                c.insert(j as i32);
            }
        }

        println!("rows: {:?}", r);
        println!("cols: {:?}", c);

        let mut galaxies = vec![];
        for i in 0..input[0].len() {
            for j in 0..input.len() {
                if input[i][j] == Elem::Galaxy {
                    galaxies.push(Pos {
                        x: j as i32,
                        y: i as i32,
                    })
                }
            }
        }

        let countcombi = (galaxies.len() - 1) * galaxies.len() / 2;

        println!("{} => {}", galaxies.len(), countcombi);

        let mut sum = 0;
        let factor = 1000_000;
        for i in 0..galaxies.len() {
            for j in (i + 1)..galaxies.len() {
                let g = galaxies[i];
                let h = galaxies[j];
                let minx = g.x.min(h.x);
                let maxx = g.x.max(h.x);
                let miny = g.y.min(h.y);
                let maxy = g.y.max(h.y);
                let dist = maxx - minx + maxy - miny;
                let count_rows = c.iter().filter(|&&ri| ri > minx && ri < maxx).count() as i64;
                let count_cols = r.iter().filter(|&&ci| ci > miny && ci < maxy).count() as i64;

                let fd = dist as i64 + count_rows * (factor - 1) + count_cols * (factor - 1);

                println!(
                    "dist {:?} to {:?} => {} ({}, {}, {})",
                    g, h, fd, dist, count_rows, count_cols
                );
                sum += fd;
            }
        }
        Ok(sum)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::Solver;

    use super::Solution;

    #[test]
    fn d11() {
        let i = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

        dbg!(Solution::solve(i, false));
    }
}
