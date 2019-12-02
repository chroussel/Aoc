use failure::Error;
use crate::solutions::Solver;

pub enum Solution {}

impl Solution {
    fn run(input: &[i32]) -> i32{
        let mut input = input.to_vec();
        let mut pc = 0;
        loop {
            let op_code = input[pc];
            match op_code {
                1 => {
                    let v1 = input[pc+1] as usize;
                    let v2 = input[pc+2] as usize;
                    let oi = input[pc+3] as usize;
                    pc+=4;
                    input[oi] = input[v1] + input[v2];
                },
                2 => {
                    let v1 = input[pc+1] as usize;
                    let v2 = input[pc+2] as usize;
                    let oi = input[pc+3] as usize;
                    pc+=4;
                    input[oi] = input[v1] * input[v2];
                }
                99 => {
                    break
                }
                _ => {unimplemented!()}
            }
        }
        input[0]
    }
}

impl Solver for Solution {
    type Input = Vec<i32>;
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, Error> {
        input.trim_end()
            .split(',')
            .filter(|s|!s.is_empty())
            .map(|u|u.parse().map_err(From::from))
            .collect()
    }

    fn solve_part1(mut input: Self::Input) -> Result<Self::Output, Error> {
        input[1] = 12;
        input[2] = 2;
        Ok(Solution::run(&input))
    }

    fn solve_part2(mut input: Self::Input) -> Result<Self::Output, Error> {
        let target = 19690720;
        for verb in 0..100 {
            for noun in 0..100 {
                input[1] = noun;
                input[2] = verb;
                let res = Solution::run(&input);
                if res == target {
                    return Ok(100 * noun + verb);
                }
            }
        }
        Ok(0)
    }
}