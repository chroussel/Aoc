use itertools::Itertools;

use crate::solutions::Solver;

use crate::solutions::AocError;

pub enum Solution {}

impl Solution {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Elem {
    Ash,
    Rock,
}

pub struct Map(Vec<Vec<Elem>>);

impl Map {
    fn iter_col(&self, col: usize) -> impl Iterator<Item = &Elem> {
        self.0[col].iter()
    }

    fn iter_row<'a>(&'a self, row: usize) -> impl Iterator<Item = &'a Elem> {
        self.0.iter().map(move |r| &r[row])
    }

    fn shape(&self) -> (usize, usize) {
        (
            self.0.len(),
            self.0.get(0).map(|r| r.len()).unwrap_or_default(),
        )
    }
}

impl Solver for Solution {
    type Input = Vec<Map>;
    type Output = usize;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        let mut maps = vec![];

        let mut current_map = vec![];
        for i in input.lines() {
            if i == "" {
                maps.push(Map(current_map));
                current_map = vec![];
            } else {
                let li = i
                    .chars()
                    .filter_map(|c| match c {
                        '.' => Some(Elem::Ash),
                        '#' => Some(Elem::Rock),
                        _ => None,
                    })
                    .collect_vec();
                current_map.push(li);
            }
        }
        if current_map.len() > 0 {
            maps.push(Map(current_map));
        }

        Ok(maps)
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, AocError> {
        let mut s = 0;
        for m in input {
            let (lc, lr) = m.shape();
            let mut col = None;
            let mut row = None;
            // mirror line
            let mut possible_col = vec![];
            for i in 0..(lc - 1) {
                let r1 = m.iter_col(i);
                let r2 = m.iter_col(i + 1);

                if r1.zip_eq(r2).all(|(a, b)| a == b) {
                    possible_col.push(i);
                }
            }
            for pc in possible_col {
                println!("possible col: {}", pc);
                let mut found = true;
                for i in 0..pc {
                    let other = pc * 2 + 1 - i;
                    if other >= lc {
                        continue;
                    }
                    let r1 = m.iter_col(i);
                    let r2 = m.iter_col(other);
                    if !r1.zip_eq(r2).all(|(a, b)| a == b) {
                        found = false;
                        break;
                    }
                }

                if found {
                    col = Some(pc + 1);
                }
            }

            let mut possible_row = vec![];
            for i in 0..(lr - 1) {
                let r1 = m.iter_row(i);
                let r2 = m.iter_row(i + 1);

                if r1.zip_eq(r2).all(|(a, b)| a == b) {
                    possible_row.push(i);
                }
            }
            for pr in possible_row {
                println!("possible row : {}", pr);

                let mut found = true;
                for i in 0..pr {
                    let other = pr * 2 + 1 - i;
                    if other >= lr {
                        continue;
                    }
                    let r1 = m.iter_row(i);
                    let r2 = m.iter_row(other);
                    if !r1.zip_eq(r2).all(|(a, b)| a == b) {
                        found = false;
                        break;
                    }
                }

                if found {
                    row = Some(pr + 1);
                }
            }

            let res = match (col, row) {
                (Some(col), None) => col * 100,
                (None, Some(row)) => row,
                (Some(col), Some(row)) => {
                    println!("{} {}", row, col);
                    unreachable!();
                }
                (None, None) => {
                    unreachable!();
                }
            };

            println!("{}", res);
            s += res;
        }

        return Ok(s);
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, AocError> {
        let mut s = 0;
        for m in input {
            let (lc, lr) = m.shape();
            let mut col = None;
            let mut row = None;
            // mirror line
            let mut possible_col = vec![];

            for i in 0..(lc - 1) {
                let r1 = m.iter_col(i);
                let r2 = m.iter_col(i + 1);

                let diff = r1.zip_eq(r2).filter(|(a, b)| a != b).count();

                if diff == 0 || diff == 1 {
                    possible_col.push((i, diff));
                }
            }
            for (pc, c) in possible_col {
                println!("possible col: {} with {} diff", pc, c);
                let mut count = c;

                for i in 0..pc {
                    let other = pc * 2 + 1 - i;
                    if other >= lc {
                        continue;
                    }
                    let r1 = m.iter_col(i);
                    let r2 = m.iter_col(other);
                    count += r1.zip_eq(r2).filter(|(a, b)| a != b).count();
                }

                if count == 1 {
                    col = Some(pc + 1);
                }
            }

            let mut possible_row = vec![];
            for i in 0..(lr - 1) {
                let r1 = m.iter_row(i);
                let r2 = m.iter_row(i + 1);

                let diff = r1.zip_eq(r2).filter(|(a, b)| a != b).count();
                if diff == 0 || diff == 1 {
                    possible_row.push((i, diff));
                }
            }
            for (pr, c) in possible_row {
                println!("possible row : {} with {} diff", pr, c);

                let mut count = c;
                for i in 0..pr {
                    let other = pr * 2 + 1 - i;
                    if other >= lr {
                        continue;
                    }
                    let r1 = m.iter_row(i);
                    let r2 = m.iter_row(other);
                    let diff = r1.zip_eq(r2).filter(|(a, b)| a != b).count();
                    count += diff;
                }

                if count == 1 {
                    row = Some(pr + 1);
                }
            }

            let res = match (col, row) {
                (Some(col), None) => col * 100,
                (None, Some(row)) => row,
                (Some(col), Some(row)) => {
                    println!("{} {}", row, col);
                    unreachable!();
                }
                (None, None) => {
                    unreachable!();
                }
            };

            println!("{}", res);
            s += res;
        }

        return Ok(s);
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::Solver;

    use super::Solution;

    #[test]
    fn d13() {
        let i = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        let i2 = "...##....
.#.##.#..
.#....#..
.##..##..
...##....
..####.##
#..##..##";

        dbg!(Solution::solve(i, false));
    }
}
