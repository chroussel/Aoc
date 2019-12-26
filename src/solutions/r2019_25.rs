use failure::Error;
use crate::solutions::Solver;

pub enum Solution {}

impl Solution {

}

impl Solver for Solution {
    type Input = Vec<i32>;
    type Output = i32;

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

#[cfg(test)]
mod tests {

}