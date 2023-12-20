use itertools::Itertools;

use crate::solutions::Solver;

use crate::solutions::AocError;

pub enum Solution {}

impl Solution {}

impl Solver for Solution {
    type Input = (Vec<i32>, Vec<i32>);
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        let mut l = input.lines();
        let (_, c) = l.next().unwrap().split_once(":").unwrap();
        let times = c
            .split_whitespace()
            .map(|e| e.parse().unwrap())
            .collect_vec();
        let (_, c) = l.next().unwrap().split_once(":").unwrap();
        let speed = c
            .split_whitespace()
            .map(|e| e.parse().unwrap())
            .collect_vec();

        Ok((times, speed))
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, AocError> {
        // D = v * (FT - t)
        // v = t * 1
        // D <= t * (FT - t) => t*FT - t2 => t2 - t*FT + D <= 0 => FT +- sqrt(FT2-4FT) / 2
        // D' = FT - 2t => t = FT/2
        //
        //
        let mut res = 1;

        let (time, dist) = input;
        for i in 0..time.len() {
            let t = time[i];
            let d = dist[i];

            let maxt = (((t as f32) + ((t * t - 4 * d) as f32).sqrt()) / 2.).floor() as i32;
            let mint = (((t as f32) - ((t * t - 4 * d) as f32).sqrt()) / 2.).ceil() as i32;

            println!("t: {} {}", mint, maxt);
            let maxd = maxt * (t - maxt);
            let mind = mint * (t - mint);
            println!("d: {} {}", mind, maxd);

            let mut count = maxt - mint + 1;
            if mind == d {
                count -= 1;
            }
            if maxd == d {
                count -= 1;
            }

            println!("s: {}", count);
            res *= count;
        }

        Ok(res)
    }

    fn solve_part2(_input: Self::Input) -> Result<Self::Output, AocError> {
        let t = 54708275;
        let d = 239114212951253;

        let maxt = (((t as f64) + ((t * t - 4 * d) as f64).sqrt()) / 2.).floor() as i64;
        let mint = (((t as f64) - ((t * t - 4 * d) as f64).sqrt()) / 2.).ceil() as i64;

        println!("t: {} {}", mint, maxt);
        let maxd = maxt * (t - maxt);
        let mind = mint * (t - mint);
        println!("d: {} {}", mind, maxd);

        let mut count = maxt - mint + 1;
        if mind == d {
            count -= 1;
        }
        if maxd == d {
            count -= 1;
        }
        return Ok(count as i32);
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::Solver;

    use super::Solution;

    #[test]
    fn d6() {
        let i = "Time:      7  15   30
Distance:  9  40  200";

        let res = Solution::solve(i, true).unwrap();
        dbg!(res);
    }
}
