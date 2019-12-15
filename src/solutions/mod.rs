mod r2018_1;
mod r2019_1;
mod r2019_2;
mod r2019_3;
mod r2019_4;
mod r2019_5;
mod r2019_6;
mod r2019_7;
mod r2019_8;
mod r2019_9;
mod r2019_10;
mod r2019_11;
mod r2019_12;
mod r2019_13;
mod r2019_14;
mod r2019_15;
mod template;
mod intcode;
mod common;

use failure::Error;
use std::env::VarError::NotPresent;

pub trait Solver {
    type Input;
    type Output: std::fmt::Display;

    fn parse_input(_: &str) -> Result<Self::Input, Error>;
    fn solve_part1(_: Self::Input) -> Result<Self::Output, Error>;
    fn solve_part2(_: Self::Input) -> Result<Self::Output, Error>;

    fn solve(input: &str, part1: bool) -> Result<String, Error>{
        let i = Self::parse_input(input)?;
        let res = if part1 {
            Self::solve_part1(i)?
        } else {
            Self::solve_part2(i)?
        };

        Ok(res.to_string())
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct InputDay {
    year: String,
    day: String,
    part: String
}

impl InputDay {
    fn new(year: &str, day: &str, part: &str) -> Self {
        InputDay {
            year: year.to_string(),
            day:day.to_string(),
            part: part.to_string()
        }
    }
}

pub fn run(year: &str, day: &str, part1: bool, input: &str) -> Result<String, Error> {
    match (year, day) {
        ("2018", "1") => r2018_1::Solution::solve(input, part1),
        ("2019", "1") => r2019_1::Solution::solve(input, part1),
        ("2019", "2") => r2019_2::Solution::solve(input, part1),
        ("2019", "3") => r2019_3::Solution::solve(input, part1),
        ("2019", "4") => r2019_4::Solution::solve(input, part1),
        ("2019", "5") => r2019_5::Solution::solve(input, part1),
        ("2019", "6") => r2019_6::Solution::solve(input, part1),
        ("2019", "7") => r2019_7::Solution::solve(input, part1),
        ("2019", "8") => r2019_8::Solution::solve(input, part1),
        ("2019", "9") => r2019_9::Solution::solve(input, part1),
        ("2019", "10") => r2019_10::Solution::solve(input, part1),
        ("2019", "11") => r2019_11::Solution::solve(input, part1),
        ("2019", "12") => r2019_12::Solution::solve(input, part1),
        ("2019", "13") => r2019_13::Solution::solve(input, part1),
        ("2019", "14") => r2019_14::Solution::solve(input, part1),
        ("2019", "15") => r2019_15::Solution::solve(input, part1),
        _ => Err(Error::from(NotPresent))
    }
}