use failure::Error;
use crate::solutions::Solver;
use itertools::Itertools;

pub enum Solution {}

impl Solution {
    fn build_matrix(size: usize) -> Vec<Vec<i32>> {
        let mut rows = vec![];
        for i in 0..size {
            let mut col = vec![0; size];
            for j in i..size {
                let v = ((j+1) / (i + 1))% 4;
                let value = match v {
                    1 => 1,
                    3 => -1,
                    _ => 0
                };
                col[j] = value;
            }
            rows.push(col);
        }
        rows
    }

    fn multiply_matrix(m0: &Vec<Vec<i32>>, m1: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut r = vec![];
        for i in 0..m0.len() {
            let mut col = vec![0; m0.len()];
            for j in 0..m0.len() {
                let mut v = 0;
                for k in 0..m0.len() {
                    v += m0[k][j] * m1[i][k];
                }
                col[j] = v;
            }
            r.push(col);
        }
        r
    }

    fn power_matrix(m: Vec<Vec<i32>>, n: usize) -> Vec<Vec<i32>>{
        let mut res = m;
        for p in 0..(n-1) {
            res = Solution::multiply_matrix(&res, &res);
        }
        res
    }

    fn phase_power(input:Vec<i32>, n: usize) -> Vec<i32> {
        let m = Solution::power_matrix(Solution::build_matrix(input.len()), n);
        println!("matrix built");
        let mut res = vec![];
        res.reserve_exact(input.len());
        for i in 0..input.len() {
            let mut r = 0;
            for (index, v1) in input[0..input.len()].iter().enumerate() {
                r += m[i][index] * v1;
            }
            r = (r % 10).abs();
            res.push(r);
        }
        res
    }

    fn phase(input: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        res.reserve_exact(input.len());
        for i in 0..input.len() {
            let mut r = 0;
            for (index, v1) in input[i..input.len()].iter().enumerate() {
                let j = (index + i + 1) / (i + 1) % 4;
                match j {
                    1 => r = (r + v1),
                    3 => r = (r - v1),
                    _ => {}
                }

            }
            r = (r % 10).abs();
            res.push(r);
        }
        res
    }
}

impl Solver for Solution {
    type Input = Vec<i32>;
    type Output = String;

    fn parse_input(input: &str) -> Result<Self::Input, Error> {
        Ok(input.chars().into_iter().filter_map(|c| c.to_digit(10).map(|i| i as i32)).collect_vec())
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, Error> {
        let mut res = input;
        for _ in 0..100 {
            res = Solution::phase(res);
        }

        Ok(res.iter().take(8).join("").to_string())
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, Error> {
        let input = input.repeat(10000);
        let mut res= input.clone();
        let skip: usize = input[0..7].iter().join("").parse().unwrap();
        for _ in 0..100 {
            for i in (skip..(res.len() - 2)).rev() {
                res[i] = (res[i] + res[i + 1]) % 10;
            }
        }
        Ok(res[skip..(skip+8)].to_vec().iter().join("").to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::r2019_16::Solution;
    use itertools::Itertools;

    #[test]
    fn e1() {
        let i0 = vec![1,2,3,4,5,6,7,8];
        let i1 = Solution::phase(i0);
        dbg!(i1.iter().join(""));
    }

    #[test]
    fn e1bis() {
        let i0 = vec![1,2,3,4,5,6,7,8];
        let i1 = Solution::phase_power(i0, 1);
        dbg!(i1.iter().join(""));
    }

    #[test]
    fn e2() {
        let m = Solution::build_matrix(18);
        for i in 0..m.len() {
            for j in 0..m[0].len() {
                print!("{:3}",m[i][j]);
            }
            println!()
        }
    }

    #[test]
    fn e3() {
        let m = Solution::power_matrix(Solution::build_matrix(8), 2);
        for i in 0..m.len() {
            for j in 0..m[0].len() {
                print!("{:3}",m[i][j]);
            }
            println!()
        }
    }
}