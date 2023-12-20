use std::collections::HashMap;

use itertools::Itertools;
use num::integer::lcm;

use crate::solutions::Solver;

use crate::solutions::AocError;

pub enum Solution {}

impl Solution {
    fn solve_start(pos: &str, input: &Data) -> usize {
        let mut p = pos;
        let mut count = 0;
        while !p.ends_with('Z') {
            let (left, right) = input.paths.get(p).unwrap();
            let adj = count % input.directions.len();
            let dir = input.directions.get(adj).unwrap();
            p = match dir {
                Dir::Right => right,
                Dir::Left => left,
            };
            count += 1;
        }
        return count;
    }
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Left,
    Right,
}

pub struct Data {
    directions: Vec<Dir>,
    paths: HashMap<String, (String, String)>,
}

impl Solver for Solution {
    type Input = Data;
    type Output = usize;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        let mut lines = input.lines();
        let dd = lines
            .next()
            .unwrap()
            .chars()
            .filter_map(|c| match c {
                'L' => Some(Dir::Left),
                'R' => Some(Dir::Right),
                _ => None,
            })
            .collect_vec();

        let mut paths = HashMap::new();
        let re = regex::Regex::new(r"(...) = \((...), (...)\)").unwrap();
        for l in lines {
            if let Some(cap) = re.captures(l) {
                let (_, [c1, c2, c3]) = cap.extract();
                paths.insert(c1.to_string(), (c2.to_string(), c3.to_string()));
            }
        }

        Ok(Data {
            directions: dd,
            paths,
        })
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, AocError> {
        let pos = "AAA";
        let count = Solution::solve_start(pos, &input);
        Ok(count)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, AocError> {
        let mut starting_pos = vec![];
        for pos in input.paths.keys() {
            if pos.ends_with('A') {
                starting_pos.push(pos.clone());
            }
        }

        println!("len: {}", starting_pos.len());

        let mut count = 1;
        for pos in starting_pos {
            let o = Solution::solve_start(&pos, &input);
            println!("{}", o);
            count = lcm(count, o)
        }

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::Solver;

    use super::Solution;

    #[test]
    fn d8() {
        let i = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
            ";

        dbg!(Solution::solve(i, true));
    }
}
