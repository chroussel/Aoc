use std::collections::HashMap;
use std::fmt::Display;

use itertools::Itertools;

use crate::solutions::Solver;

use crate::solutions::common::Map;
use crate::solutions::AocError;

pub enum Solution {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Elem {
    RoundRock,
    CubeRock,
    Empty,
}

impl Display for Elem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Elem::Empty => write!(f, ".")?,
            Elem::CubeRock => write!(f, "#")?,
            Elem::RoundRock => write!(f, "O")?,
        }
        Ok(())
    }
}

impl Elem {
    fn from_char(c: char) -> Option<Elem> {
        match c {
            '.' => Some(Elem::Empty),
            '#' => Some(Elem::CubeRock),
            'O' => Some(Elem::RoundRock),
            _ => None,
        }
    }
}

impl Solution {
    fn tilt_north(map: &Map<Elem>) -> Map<Elem> {
        let (rows, cols) = map.shape();

        let mut tilted = vec![vec![Elem::Empty; cols]; rows];
        for i in 0..cols {
            let col = map.iter_col(i);
            let mut index_mut = 0;
            for (j, c) in col.enumerate() {
                match c {
                    Elem::Empty => continue,
                    Elem::RoundRock => {
                        tilted[index_mut][i] = Elem::RoundRock;
                        index_mut += 1;
                    }
                    Elem::CubeRock => {
                        tilted[j][i] = Elem::CubeRock;
                        index_mut = j + 1;
                    }
                }
            }
        }
        Map(tilted)
    }

    fn tilt_west(map: &Map<Elem>) -> Map<Elem> {
        let (rows, cols) = map.shape();

        let mut tilted = vec![vec![Elem::Empty; cols]; rows];
        for i in 0..rows {
            let col = map.iter_row(i);
            let mut index_mut = 0;
            for (j, c) in col.enumerate() {
                match c {
                    Elem::Empty => continue,
                    Elem::RoundRock => {
                        tilted[i][index_mut] = Elem::RoundRock;
                        index_mut += 1;
                    }
                    Elem::CubeRock => {
                        tilted[i][j] = Elem::CubeRock;
                        index_mut = j + 1;
                    }
                }
            }
        }
        Map(tilted)
    }

    fn tilt_east(map: &Map<Elem>) -> Map<Elem> {
        let (rows, cols) = map.shape();

        let mut tilted = vec![vec![Elem::Empty; cols]; rows];
        for i in 0..rows {
            let col = map.iter_row(i);
            let mut index_mut = cols - 1;
            for (j, c) in col.enumerate().rev() {
                match c {
                    Elem::Empty => continue,
                    Elem::RoundRock => {
                        tilted[i][index_mut] = Elem::RoundRock;
                        if index_mut > 0 {
                            index_mut -= 1;
                        }
                    }
                    Elem::CubeRock => {
                        tilted[i][j] = Elem::CubeRock;
                        if j > 0 {
                            index_mut = j - 1;
                        }
                    }
                }
            }
        }
        Map(tilted)
    }

    fn tilt_south(map: &Map<Elem>) -> Map<Elem> {
        let (rows, cols) = map.shape();

        let mut tilted = vec![vec![Elem::Empty; cols]; rows];
        for i in 0..cols {
            let col = map.iter_col(i);
            let mut index_mut = rows - 1;
            for (j, c) in col.enumerate().rev() {
                match c {
                    Elem::Empty => continue,
                    Elem::RoundRock => {
                        tilted[index_mut][i] = Elem::RoundRock;
                        if index_mut > 0 {
                            index_mut -= 1;
                        }
                    }
                    Elem::CubeRock => {
                        tilted[j][i] = Elem::CubeRock;
                        if j > 0 {
                            index_mut = j - 1;
                        }
                    }
                }
            }
        }
        Map(tilted)
    }

    fn cycle(map: &Map<Elem>) -> Map<Elem> {
        let mut map = Solution::tilt_north(map);
        map = Solution::tilt_west(&map);
        map = Solution::tilt_south(&map);
        map = Solution::tilt_east(&map);
        map
    }

    fn score(map: &Map<Elem>) -> usize {
        let mut res = 0;
        let (rows, cols) = map.shape();
        for i in 0..rows {
            let score = rows - i;
            let count = map.iter_row(i).filter(|e| **e == Elem::RoundRock).count();
            res += count * score;
        }
        return res;
    }
}

impl Solver for Solution {
    type Input = Map<Elem>;
    type Output = usize;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        let r = input
            .lines()
            .map(|l| l.chars().filter_map(|c| Elem::from_char(c)).collect_vec())
            .collect_vec();
        Ok(Map(r))
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, AocError> {
        let tilted = Solution::tilt_north(&input);
        let res = Solution::score(&tilted);
        Ok(res)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, AocError> {
        let cycles = 1000000000;
        let mut tilted = input.clone();

        let mut cache = HashMap::new();
        cache.insert(input.clone(), (0, Solution::score(&input)));

        let mut c = 0;

        tilted = Solution::cycle(&tilted);
        c += 1;
        while cache.get(&tilted).is_none() {
            let score = Solution::score(&tilted);
            cache.insert(tilted.clone(), (c, score));

            tilted = Solution::cycle(&tilted);
            c += 1;
        }

        let (prev_cycle, _) = cache.get(&tilted).unwrap();
        let current = c;

        let modres = (cycles - prev_cycle) % (current - prev_cycle);

        println!(
            "{} {} {} {}",
            prev_cycle,
            current,
            current - prev_cycle,
            modres
        );
        let mut mapping = HashMap::new();
        for (k, v) in cache.values() {
            mapping.insert(*k, *v);
        }

        println!("{:?}", mapping);

        let res = mapping.get(&(prev_cycle + modres)).unwrap();
        Ok(*res)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::Solver;

    use super::Solution;

    #[test]
    fn d14() {
        let i = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        dbg!(Solution::solve(i, false));
    }
}
