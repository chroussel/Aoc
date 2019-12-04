use failure::Error;
use crate::solutions::Solver;
use std::collections::HashMap;

pub enum Solution {}

impl Solution {
    fn is_valid(pass: i32, part1: bool) -> bool {
        let digits = Solution::convert_to_digit(pass);
        // contains double
        let mut map = HashMap::new();

        digits.iter().for_each(|d| {
           map.entry(d).and_modify(|e| *e+=1).or_insert(1);
        });

        let mut has_double = false;
        for (_,v) in map {
            if part1 {
                if v > 1 {
                    has_double = true;
                    break
                }
            } else {
                if v == 2 {
                    has_double = true;
                    break
                }
            }

        }

        // only increasing
        let mut onlyI = true;
        for i in 0..5 {
            onlyI &= digits[i] <= digits[i+1]
        }
        has_double && onlyI
    }

    fn convert_to_digit(mut number: i32) -> Vec<i32> {
        let mut res = vec![];
        while (number > 0) {
            let d = number % 10;
            res.insert(0, d);
            number /= 10
        }
        res
    }
}

impl Solver for Solution {
    type Input = (i32, i32);
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, Error> {
        let r: Result<Vec<i32>, Error> =
            input.trim_end().split("-")
            .map(|s|s.parse::<i32>().map_err(From::from)).collect();
        let r = r?;
        Ok((r[0], r[1]))
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, Error> {
        let (low, high) = input;
        let res = (low..=high).filter(|&d|Solution::is_valid(d, true)).count();
        Ok(res as i32)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, Error> {
        let (low, high) = input;
        let res = (low..=high).filter(|&d|Solution::is_valid(d, false)).count();
        Ok(res as i32)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::r2019_4::Solution;

    #[test]
    fn is_valid_e1() {
        assert_eq!(false, Solution::is_valid(223450, true))
    }

    #[test]
    fn is_valid_e2() {
        assert_eq!(true, Solution::is_valid(111111, true))
    }

    #[test]
    fn is_valid_e3() {
        assert_eq!(false, Solution::is_valid(123789, true))
    }

    #[test]
    fn is_valid_e4() {
        assert_eq!(false, Solution::is_valid(111111, false));
        assert_eq!(false, Solution::is_valid(123789, false));
        assert_eq!(true, Solution::is_valid(112233, false));
        assert_eq!(false, Solution::is_valid(123444, false));
    }
    #[test]
    fn is_valid_e5() {
        assert_eq!(true, Solution::is_valid(111122, false));
    }
}