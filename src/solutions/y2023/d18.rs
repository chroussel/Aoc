use itertools::Itertools;

use crate::solutions::Solver;

use crate::solutions::common::Map;
use crate::solutions::AocError;

pub enum Solution {}

impl Solution {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    mov: Dir,
    length: i32,
    color: String,
}

impl Solver for Solution {
    type Input = Vec<Move>;
    type Output = i64;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        let lls = input
            .lines()
            .map(|l| {
                let (m, s, c) = l.splitn(3, ' ').collect_tuple().unwrap();

                let mov = match m {
                    "R" => Dir::Right,
                    "L" => Dir::Left,
                    "U" => Dir::Up,
                    "D" => Dir::Down,
                    _ => unimplemented!(),
                };

                Move {
                    mov,
                    length: s.parse().unwrap(),
                    color: c.into(),
                }
            })
            .collect_vec();

        Ok(lls)
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, AocError> {
        let mut dimx = 0;
        let mut dimy = 0;
        let mut curx = 0;
        let mut cury = 0;
        let mut mx = 0;
        let mut my = 0;

        for mo in input.iter() {
            match mo.mov {
                Dir::Right => curx += mo.length,
                Dir::Left => curx -= mo.length,
                Dir::Up => cury -= mo.length,
                Dir::Down => cury += mo.length,
            }
            dimx = dimx.max(curx + 1);
            dimy = dimy.max(cury + 1);
            mx = mx.min(curx);
            my = my.min(cury);
        }
        println!("{} {} {} {}", mx, my, dimx, dimy);
        let shiftedx = -mx;
        let shiftedy = -my;

        let mut map = Map(vec![
            vec!["."; (shiftedx + dimx) as usize];
            (shiftedy + dimy) as usize
        ]);

        let mut surf = 0;
        let mut pos = (shiftedy, shiftedx);
        let mut edges = 0;
        for mo in input.iter() {
            let prevpos = pos.clone();
            let p = match mo.mov {
                Dir::Right => (0, 1),
                Dir::Left => (0, -1),
                Dir::Up => (-1, 0),
                Dir::Down => (1, 0),
            };

            for i in 0..mo.length {
                pos = (pos.0 + p.0, pos.1 + p.1);
                map.0[pos.0 as usize][pos.1 as usize] = "#";
                edges += 1;
            }
            surf += prevpos.1 * (pos.0) - (pos.1) * prevpos.0;
        }
        surf = surf / 2;
        let outside = edges;
        let p = surf + outside / 2 + 1;
        map.print_map();
        println!("{} {} => {}", surf, outside, p);
        Ok(p as i64)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, AocError> {
        let mut surf = 0;
        let mut pos = (0, 0);
        let mut edges = 0;
        for mo in input.iter() {
            let prevpos = pos.clone();

            let mut chars = mo.color.chars();
            let mut length = 0;

            let mut dir = Dir::Right;

            for (i, c) in mo.color.chars().skip(2).take(6).enumerate() {
                if i == 5 {
                    dir = match c {
                        '0' => Dir::Right,
                        '1' => Dir::Down,
                        '2' => Dir::Left,
                        '3' => Dir::Up,
                        _ => unimplemented!(),
                    }
                } else {
                    length = length * 16 + c.to_digit(16).unwrap() as i64;
                }
            }

            println!("{:?} {}", dir, length);

            let p = match dir {
                Dir::Right => (0, length),
                Dir::Left => (0, -length),
                Dir::Up => (-length, 0),
                Dir::Down => (length, 0),
            };

            edges += length;
            pos = (pos.0 + p.0, pos.1 + p.1);
            surf += prevpos.1 * (pos.0) - (pos.1) * prevpos.0;
        }
        surf = surf / 2;
        let outside = edges;
        let p = surf + outside / 2 + 1;
        println!("{} {} => {}", surf, outside, p);
        Ok(p)
    }
}

#[cfg(test)]
mod tests {

    use crate::solutions::Solver;

    use super::Solution;
    #[test]
    fn d18() {
        let i = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";
        let i2 = "R 2 (#000000)
D 2 (#000000)
L 2 (#000000)
U 2 (#000000)
";

        dbg!(Solution::solve(i, true));
        dbg!(Solution::solve(i, false));
    }
}
