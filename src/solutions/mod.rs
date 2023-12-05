use thiserror::Error;

mod common;
mod template;
mod y2023;

#[derive(Debug, Error)]
pub enum AocError {
    #[error("parse eror")]
    ParseIntError {
        #[from]
        source: std::num::ParseIntError,
    },
    #[error("unknown error {0}")]
    Unknown(String),
}

pub trait Solver {
    type Input;
    type Output: std::fmt::Display;

    fn parse_input(_: &str) -> Result<Self::Input, AocError>;
    fn solve_part1(_: Self::Input) -> Result<Self::Output, AocError>;
    fn solve_part2(_: Self::Input) -> Result<Self::Output, AocError>;

    fn solve(input: &str, part1: bool) -> Result<String, AocError> {
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
    part: String,
}

impl InputDay {
    fn new(year: &str, day: &str, part: &str) -> Self {
        InputDay {
            year: year.to_string(),
            day: day.to_string(),
            part: part.to_string(),
        }
    }
}

pub fn run(year: &str, day: &str, part1: bool, input: &str) -> Result<String, AocError> {
    match (year, day) {
        ("2023", "1") => y2023::d1::Solution::solve(input, part1),
        ("2023", "2") => y2023::d2::Solution::solve(input, part1),
        ("2023", "3") => y2023::d3::Solution::solve(input, part1),
        ("2023", "4") => y2023::d4::Solution::solve(input, part1),
        ("2023", "5") => y2023::d5::Solution::solve(input, part1),
        ("2023", "6") => y2023::d6::Solution::solve(input, part1),
        ("2023", "7") => y2023::d7::Solution::solve(input, part1),
        ("2023", "8") => y2023::d8::Solution::solve(input, part1),
        ("2023", "9") => y2023::d9::Solution::solve(input, part1),
        ("2023", "10") => y2023::d10::Solution::solve(input, part1),
        ("2023", "11") => y2023::d11::Solution::solve(input, part1),
        ("2023", "12") => y2023::d12::Solution::solve(input, part1),
        ("2023", "13") => y2023::d13::Solution::solve(input, part1),
        ("2023", "14") => y2023::d14::Solution::solve(input, part1),
        ("2023", "15") => y2023::d15::Solution::solve(input, part1),
        ("2023", "16") => y2023::d16::Solution::solve(input, part1),
        ("2023", "17") => y2023::d17::Solution::solve(input, part1),
        ("2023", "18") => y2023::d18::Solution::solve(input, part1),
        ("2023", "19") => y2023::d19::Solution::solve(input, part1),
        ("2023", "20") => y2023::d20::Solution::solve(input, part1),
        ("2023", "21") => y2023::d21::Solution::solve(input, part1),
        ("2023", "22") => y2023::d22::Solution::solve(input, part1),
        ("2023", "23") => y2023::d23::Solution::solve(input, part1),
        ("2023", "24") => y2023::d24::Solution::solve(input, part1),
        ("2023", "25") => y2023::d25::Solution::solve(input, part1),
        _ => unimplemented!(),
    }
}
