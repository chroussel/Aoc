use thiserror::Error;

mod y2021;
mod template;
mod common;

#[derive(Debug, Error)]
pub enum AocError {
    #[error("parse eror")]
    ParseIntError{
        #[from]
        source: std::num::ParseIntError
    },
    #[error("unknown error {0}")]
    Unknown(String)
}

pub trait Solver {
    type Input;
    type Output: std::fmt::Display;

    fn parse_input(_: &str) -> Result<Self::Input, AocError>;
    fn solve_part1(_: Self::Input) -> Result<Self::Output, AocError>;
    fn solve_part2(_: Self::Input) -> Result<Self::Output, AocError>;

    fn solve(input: &str, part1: bool) -> Result<String, AocError>{
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

pub fn run(year: &str, day: &str, part1: bool, input: &str) -> Result<String, AocError> {
    match (year, day) {
         ("2021","1") => y2021::d1::Solution::solve(input, part1),
         ("2021","2") => y2021::d2::Solution::solve(input, part1),
         _ => unimplemented!()
    }
}