use failure::Error;
use crate::solutions::Solver;

pub enum Solution {}

impl Solver for Solution {
    type Input = Vec<i32>;
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, Error> {
        input.lines()
            .map(|l|l.parse().map_err(From::from))
            .collect()
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, Error> {
        Ok(input.iter().sum())
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, Error> {
        let mut hash_set = std::collections::HashSet::new();
        let mut running = 0;

        for vi in input.iter().cycle() {
            running += vi;
            if hash_set.contains(&running) {
                return Ok(running)
            } else {
                hash_set.insert(running);
            }
        }
        unreachable!()
    }
}