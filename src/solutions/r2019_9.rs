use failure::Error;
use crate::solutions::Solver;
use crate::solutions::intcode::Program;

pub enum Solution {}

impl Solver for Solution {
    type Input = Vec<i64>;
    type Output = i64;

    fn parse_input(input: &str) -> Result<Self::Input, Error> {
        input.trim_end()
            .split(',')
            .filter(|s|!s.is_empty())
            .map(|u|u.parse().map_err(From::from))
            .collect()
    }

    fn solve_part1(mut input: Self::Input) -> Result<Self::Output, Error> {
        let res = Program::new(input).run_with_input(vec!(1));
        dbg!(&res);
        Ok(*res.last().unwrap())
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, Error> {
        let res = Program::new(input).run_with_input(vec!(1));
        dbg!(&res);
        Ok(*res.last().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::r2019_9::Solution;
    use crate::solutions::Solver;

    #[test]
    fn e1() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut code = Solution::parse_input(input).unwrap();
        let res = Solution::run(&code, vec!());
        dbg!(res);
    }

    #[test]
    fn e2() {
        let input = "104,1125899906842624,99";
        let mut code = Solution::parse_input(input).unwrap();
        code.resize(200, 0);
        let res = Solution::run(&code, vec!());
        dbg!(res);
    }

    #[test]
    fn e3() {
        let input = "1102,34915192,34915192,7,4,7,99,0";
        let mut code = Solution::parse_input(input).unwrap();
        code.resize(200, 0);
        let res = Solution::run(&code, vec!());
        dbg!(res);
    }
}