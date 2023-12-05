use std::error::Error;

use itertools::Itertools;

use crate::solutions::Solver;

use crate::solutions::AocError;

pub enum Solution {}

impl Solution {}

pub struct Move {
    red: i32,
    blue: i32,
    green: i32,
}

pub struct Game {
    id: i32,
    moves: Vec<Move>,
}

impl Solver for Solution {
    type Input = Vec<Game>;
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        let r = input
            .lines()
            .map(|l| {
                let (g, r) = l.split_once(':').unwrap();
                let (_, id) = g.split_once(' ').unwrap();
                let moves = r
                    .split(';')
                    .map(|p| {
                        let mut red = 0;
                        let mut blue = 0;
                        let mut green = 0;
                        p.trim().split(",").for_each(|part| {
                            let (pos, color) = part.trim().split_once(' ').unwrap();
                            let pos = pos.parse().unwrap();
                            match color {
                                "red" => red = pos,
                                "green" => green = pos,
                                "blue" => blue = pos,
                                _ => unimplemented!(),
                            };
                        });
                        Move { red, blue, green }
                    })
                    .collect_vec();
                Game {
                    id: id.parse().unwrap(),
                    moves,
                }
            })
            .collect_vec();
        Ok(r)
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, AocError> {
        let mut s = 0;
        for g in input {
            let mut red = 12;
            let mut green = 13;
            let mut blue = 14;
            let mut pos = true;
            for m in g.moves {
                if m.red > red || m.blue > blue || m.green > green {
                    pos = false;
                    break;
                }
            }
            if pos {
                s += g.id;
            }
        }

        Ok(s)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, AocError> {
        let mut s = 0;
        for g in input {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for m in g.moves {
                if m.red > red {
                    red = m.red;
                }
                if m.green > green {
                    green = m.green;
                }
                if m.blue > blue {
                    blue = m.blue;
                }
            }
            let power = red * blue * green;
            s += power;
        }

        Ok(s)
    }
}

#[cfg(test)]
mod tests {}
