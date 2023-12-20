use std::collections::HashMap;

use itertools::Itertools;

use crate::solutions::Solver;

use crate::solutions::AocError;

pub enum Solution {}

impl Solution {
    fn hash(s: &str) -> i32 {
        s.chars()
            .filter(|c| *c != '\n')
            .fold(0_i32, |acc, v| (acc + v as i32) * 17 % 256)
    }
}

impl Solver for Solution {
    type Input = Vec<String>;
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        Ok(input.split(',').map(|s| s.into()).collect_vec())
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, AocError> {
        let mut sum = 0;
        for step in input {
            let mut s = 0;

            let s = step
                .chars()
                .filter(|c| *c != '\n')
                .fold(0_i32, |acc, v| (acc + v as i32) * 17 % 256);
            sum += s;
        }
        Ok(sum)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, AocError> {
        let mut sum = 0;

        let mut boxes: HashMap<i32, Vec<(String, i32)>> = HashMap::new();
        for step in input {
            if let Some((p1, f)) = step.split_once('=') {
                let h = Solution::hash(p1);
                let v = boxes.entry(h).or_insert(vec![]);
                if let Some(elem) = v.iter_mut().find(|e| e.0 == p1) {
                    elem.1 = f.parse().unwrap();
                } else {
                    v.push((p1.to_owned(), f.parse().unwrap()));
                }
            } else {
                let (s, _) = step.split_once('-').unwrap();
                let h = Solution::hash(s);
                if let Some(b) = boxes.get_mut(&h) {
                    if let Some((pos, _)) = b.iter().find_position(|e| e.0 == s) {
                        b.remove(pos);
                    }
                }
            }
        }
        println!("{:?}", boxes);
        for (b, map) in boxes {
            for (pos, (label, lens)) in map.iter().enumerate() {
                println!("{}: {} {} {}", label, b, pos, lens);
                let score = (b + 1) * (pos as i32 + 1) * lens;
                sum += score;
            }
        }

        Ok(sum)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::Solver;

    use super::Solution;

    #[test]
    fn d15() {
        let i = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let res = Solution::solve(i, false);
        dbg!(res);
    }
}
