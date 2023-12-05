use itertools::Itertools;

use crate::solutions::AocError;
use crate::solutions::Solver;

pub enum Solution {}

struct Ma {
    valid: bool,
    done: bool,
    chars: Vec<char>,
    index: usize,
    digit: u32,
}

impl Ma {
    fn new(pattern: &str, digit: u32) -> Ma {
        Ma {
            valid: true,
            done: false,
            chars: pattern.chars().collect_vec(),
            index: 0,
            digit,
        }
    }

    fn digit(&self) -> Option<u32> {
        if self.done && self.valid {
            return Some(self.digit);
        }
        return None;
    }

    fn next(&mut self, c: char) -> (bool, bool) {
        if self.done {
            return (self.valid, self.done);
        }
        if self.chars[self.index] == c {
            self.index += 1;
            self.done = self.index == self.chars.len()
        } else {
            self.valid = false;
            self.done = true;
        }

        return (self.valid, self.done);
    }
}

impl Solver for Solution {
    type Input = Vec<String>;
    type Output = u32;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        Ok(input.lines().map(|l| l.into()).collect_vec())
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, AocError> {
        let r = input
            .iter()
            .map(|l| {
                let s2 = l.chars().filter_map(|c| c.to_digit(10)).collect_vec();
                let d = s2.first().unwrap() * 10 + s2.last().unwrap();
                println!("{}", &d);
                d
            })
            .sum();
        Ok(r)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, AocError> {
        let r = input
            .iter()
            .map(|l| {
                let mut current_ma: Vec<Ma> = vec![];
                let mut first = None;
                let mut last = None;
                for c in l.chars() {
                    match c {
                        '1'..='9' => {
                            if let None = first {
                                first = c.to_digit(10)
                            }
                            last = c.to_digit(10);
                        }
                        'a'..='z' => {
                            match c {
                                'o' => {
                                    current_ma.push(Ma::new("one", 1));
                                }
                                't' => {
                                    current_ma.push(Ma::new("two", 2));
                                    current_ma.push(Ma::new("three", 3));
                                }
                                'f' => {
                                    current_ma.push(Ma::new("four", 4));
                                    current_ma.push(Ma::new("five", 5));
                                }
                                's' => {
                                    current_ma.push(Ma::new("six", 6));
                                    current_ma.push(Ma::new("seven", 7));
                                }
                                'e' => {
                                    current_ma.push(Ma::new("eight", 8));
                                }
                                'n' => {
                                    current_ma.push(Ma::new("nine", 9));
                                }
                                _ => {}
                            }
                            let mut next = vec![];
                            for mut ma in current_ma {
                                if let (true, true) = ma.next(c) {
                                    if let None = first {
                                        first = ma.digit();
                                    }
                                    last = ma.digit();
                                } else {
                                    next.push(ma)
                                }
                            }
                            current_ma = next;
                        }
                        _ => {}
                    }
                }
                let d = first.unwrap() * 10 + last.unwrap();
                println!("{}", d);
                d
            })
            .sum();

        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::solutions::Solver;

    #[test]
    fn test1() {
        let i = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        let ab = Solution::parse_input(i).unwrap();
        let r = Solution::solve_part2(ab).unwrap();
        assert_eq!(r, 281);
    }

    #[test]
    fn test2() {
        let i = "three7pktwo4279z
";
        let ab = Solution::parse_input(i).unwrap();
        let r = Solution::solve_part2(ab).unwrap();
        assert_eq!(r, 39)
    }
}
