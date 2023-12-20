use itertools::Itertools;

use crate::solutions::Solver;

use crate::solutions::AocError;

pub enum Solution {}

impl Solution {
    fn derive_matrix(a: &[i32]) -> Vec<Vec<i32>> {
        let mut w = vec![vec![0; a.len() + 1]; a.len()];
        for (i, v) in a.iter().enumerate() {
            w[0][i] = *v
        }
        for i in 1..a.len() {
            for j in i..a.len() {
                w[i][j] = w[i - 1][j] - w[i - 1][j - 1]
            }
        }
        w
    }
}

impl Solver for Solution {
    type Input = Vec<Vec<i32>>;
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        let r = input
            .lines()
            .map(|l| l.split(' ').map(|c| c.parse().unwrap()).collect_vec())
            .collect_vec();
        Ok(r)
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, AocError> {
        let mut ss = 0;

        for l in input {
            let mut d = Solution::derive_matrix(&l);
            let last_i = l.len() - 1;

            for i in (0..last_i).rev() {
                d[i][l.len()] = d[i][l.len() - 1] + d[i + 1][l.len()];
            }
            ss += d[0][l.len()];
        }
        Ok(ss)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, AocError> {
        let mut ss = 0;

        for l in input {
            let mut d = Solution::derive_matrix(&l);
            for i in (0..(l.len() - 1)).rev() {
                d[i][l.len()] = d[i][i] - d[i + 1][l.len()];
            }
            ss += d[0][l.len()];
        }

        Ok(ss)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::Solver;

    use super::Solution;

    #[test]
    fn d9() {
        let i = "10 13 16 21 30 45
";

        dbg!(Solution::solve(i, false));
    }
}
