use failure::Error;
use crate::solutions::Solver;

pub enum Solution {}
impl Solution {
    fn compute_fuel(mass: i64) -> i64 {
        let f = mass.div_euclid(3) -2;
        if f <= 0 {
            0
        } else {
            Solution::compute_fuel(f) + f
        }
    }
}

impl Solver for Solution {
    type Input = Vec<i64>;
    type Output = i64;

    fn parse_input(input: &str) -> Result<Self::Input, Error> {
        input.lines()
            .map(|l| l.parse().map_err(From::from))
            .collect()
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, Error> {
        Ok(input.iter()
            .map(|a| a.div_euclid(3) - 2)
            .sum())
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, Error> {
        Ok(input.iter()
               .map(|&a|Solution::compute_fuel(a))
               .sum())
    }
}