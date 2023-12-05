

use crate::solutions::Solver;

use crate::solutions::AocError;

pub enum Solution {}

impl Solution {}

impl Solver for Solution {
    type Input = Vec<i32>;
    type Output = i32;

    fn parse_input(_input: &str) -> Result<Self::Input, AocError> {
        unimplemented!()
    }

    fn solve_part1(_input: Self::Input) -> Result<Self::Output, AocError> {
        unimplemented!()
    }

    fn solve_part2(_input: Self::Input) -> Result<Self::Output, AocError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {}
