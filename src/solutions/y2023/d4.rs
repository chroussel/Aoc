use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

use crate::solutions::Solver;

use crate::solutions::AocError;

pub enum Solution {}

impl Solution {}

#[derive(Debug, Clone)]
pub struct Card {
    id: i32,
    part1: HashSet<i32>,
    part2: HashSet<i32>,
}

impl Solver for Solution {
    type Input = Vec<Card>;
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        let r = input
            .lines()
            .map(|cl| {
                let (id, cards) = cl.split_once(':').unwrap();
                let (p1, p2) = cards.split_once('|').unwrap();

                let (_, id) = id.split_once(' ').unwrap();
                let part1 = p1
                    .split(' ')
                    .filter_map(|p| p.trim().parse::<i32>().ok())
                    .collect();
                let part2 = p2
                    .split(' ')
                    .filter_map(|p| p.trim().parse::<i32>().ok())
                    .collect();
                Card {
                    id: id.trim().parse().unwrap(),
                    part1,
                    part2,
                }
            })
            .collect_vec();

        Ok(r)
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, AocError> {
        let mut sum = 0;
        for c in input {
            let s = c.part1.intersection(&c.part2).collect_vec();
            if s.len() > 0 {
                sum += 2_i32.pow(s.len() as u32 - 1 as u32)
            }
        }
        Ok(sum)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, AocError> {
        let hm = input
            .iter()
            .map(|c| (c.id, c.clone()))
            .collect::<HashMap<_, _>>();

        let mut queue = input.clone();
        let mut res = HashMap::new();
        let mut sum = 0;
        while let Some(c) = queue.pop() {
            sum += 1;
            res.entry(c.id).and_modify(|a| *a += 1).or_insert(1);
            let s = c.part1.intersection(&c.part2).collect_vec();
            let w = s.len() as i32;
            for i in (c.id + 1)..(c.id + w + 1) {
                queue.push(hm.get(&i).cloned().unwrap());
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
    fn d4() {
        let i = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

        let r = Solution::solve(i, true).unwrap();
        dbg!(r);
    }

    #[test]
    fn d4_2() {
        let i = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

        let r = Solution::solve(i, false).unwrap();
        dbg!(r);
    }
}
