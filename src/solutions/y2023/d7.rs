use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;

use crate::solutions::Solver;

use crate::solutions::AocError;

pub enum Solution {}

impl Solution {}

pub struct Hand {
    cc: Vec<char>,
    cs: String,
    cards: HashMap<char, usize>,
    bid: i32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Comb {
    Five = 1,
    Four = 2,
    Full = 3,
    Three = 4,
    DPair = 5,
    Pair = 6,
    Single = 7,
}

impl Hand {
    fn comb(&self) -> Comb {
        let m = self.cards.iter().sorted_by_key(|a| a.1).rev().collect_vec();
        let (_, s) = m[0];
        match s {
            5 => Comb::Five,
            4 => Comb::Four,
            3 => {
                let (_, s2) = m[1];
                if *s2 == 2 {
                    Comb::Full
                } else {
                    Comb::Three
                }
            }
            2 => {
                let (_, s2) = m[1];
                if *s2 == 2 {
                    Comb::DPair
                } else {
                    Comb::Pair
                }
            }
            _ => Comb::Single,
        }
    }

    fn comb2(&self) -> Comb {
        let jcount = self.cards.get(&'J').copied().unwrap_or(0);
        let m = self.cards.iter().sorted_by_key(|a| a.1).rev().collect_vec();
        let (c, mut s) = m.get(0).copied().unwrap();
        if c == &'J' {
            (_, s) = m.get(1).copied().unwrap_or((&'Z', &0));
        }
        match s + jcount {
            5 => Comb::Five,
            4 => Comb::Four,
            3 => {
                let (_, s2) = m[1];
                if *s2 == 2 {
                    Comb::Full
                } else {
                    Comb::Three
                }
            }
            2 => {
                let (_, s2) = m[1];
                if *s2 == 2 {
                    Comb::DPair
                } else {
                    Comb::Pair
                }
            }
            _ => Comb::Single,
        }
    }
}

impl Solver for Solution {
    type Input = Vec<Hand>;
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        let r = input
            .lines()
            .map(|c| {
                let (cards, bid) = c.split_once(' ').unwrap();
                let chm = cards.chars().counts_by(|a| a);

                Hand {
                    cs: cards.to_uppercase(),
                    cc: cards.chars().collect_vec(),
                    cards: chm,
                    bid: bid.parse().unwrap(),
                }
            })
            .collect_vec();
        Ok(r)
    }

    fn solve_part1(mut input: Self::Input) -> Result<Self::Output, AocError> {
        let card_order = "23456789TJQKA";

        let card_rank: HashMap<char, usize> = card_order
            .chars()
            .rev()
            .enumerate()
            .map(|(a, b)| (b, a))
            .collect();
        input.sort_by(|a, b| {
            let c1 = a.comb();
            let c2 = b.comb();

            let cm = c1.cmp(&c2);

            if cm == Ordering::Equal {
                for i in 0..5 {
                    let ca = a.cc[i];
                    let cb = b.cc[i];
                    let cr = card_rank[&ca].cmp(&card_rank[&cb]);
                    if cr != Ordering::Equal {
                        return cr;
                    }
                }
            }
            return cm;
        });

        let mut score = 0;
        for (r, i) in input.iter().enumerate() {
            let rank = input.len() - r;
            println!("{} => {} ({:?})", &i.cs, rank, i.comb());
            score += i.bid * (rank) as i32;
        }
        Ok(score)
    }

    fn solve_part2(mut input: Self::Input) -> Result<Self::Output, AocError> {
        let card_order = "J23456789TQKA";

        let card_rank: HashMap<char, usize> = card_order
            .chars()
            .rev()
            .enumerate()
            .map(|(a, b)| (b, a))
            .collect();
        input.sort_by(|a, b| {
            let c1 = a.comb2();
            let c2 = b.comb2();

            let cm = c1.cmp(&c2);

            if cm == Ordering::Equal {
                for i in 0..5 {
                    let ca = a.cc[i];
                    let cb = b.cc[i];
                    let cr = card_rank[&ca].cmp(&card_rank[&cb]);
                    if cr != Ordering::Equal {
                        return cr;
                    }
                }
            }
            return cm;
        });

        let mut score = 0;
        for (r, i) in input.iter().enumerate() {
            let rank = input.len() - r;
            println!("{} => {} ({:?})", &i.cs, rank, i.comb2());
            score += i.bid * (rank) as i32;
        }
        Ok(score)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::Solver;

    use super::Solution;

    #[test]
    fn d7() {
        let i = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        dbg!(Solution::solve(i, true));
    }
}
