use failure::Error;
use crate::solutions::Solver;

pub enum Solution {}

impl Solver for Solution {
    type Input = Vec<i64>;
    type Output = i64;

    fn parse_input(input: &str) -> Result<Self::Input, Error> {
        unimplemented!()
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, Error> {
        unimplemented!()
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, Error> {
        unimplemented!()
    }
}