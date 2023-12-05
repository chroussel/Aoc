use crate::solutions::Solver;

use crate::solutions::AocError;

pub enum Solution {}

impl Solution {}

#[derive(Clone, Copy, Debug)]
pub enum TokenValue {
    Value(u32),
    Symbol(char),
}

#[derive(Debug)]
pub struct Token {
    startx: i32,
    endx: i32,
    y: i32,
    value: TokenValue,
}

impl Solver for Solution {
    type Input = Vec<Token>;
    type Output = u32;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        let mut y = 0;
        let mut tokens = vec![];
        for l in input.lines() {
            let mut startx = None;
            let mut chars = String::new();
            for (p, c) in l.char_indices() {
                match c {
                    '0'..='9' => {
                        if startx.is_none() {
                            startx = Some(p as i32);
                        }
                        chars.push(c);
                    }
                    s => {
                        if chars.len() > 0 {
                            tokens.push(Token {
                                startx: startx.unwrap(),
                                endx: p as i32,
                                y,
                                value: TokenValue::Value(chars.parse().unwrap()),
                            });
                            startx = None;
                            chars = String::new();
                        }
                        if c == '.' || c == '\n' {
                            continue;
                        }

                        tokens.push(Token {
                            startx: p as i32,
                            endx: p as i32,
                            y,
                            value: TokenValue::Symbol(s),
                        });
                    }
                }
            }
            if chars.len() > 0 {
                let sx = startx.unwrap();
                tokens.push(Token {
                    startx: sx,
                    endx: sx + chars.len() as i32,
                    y,
                    value: TokenValue::Value(chars.parse().unwrap()),
                });
            }
            y += 1;
        }

        Ok(tokens)
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, AocError> {
        let mut sum = 0;

        let mut values = vec![];
        let mut symbol = vec![];
        for s in input {
            match &s.value {
                TokenValue::Symbol(_) => {
                    symbol.push(s);
                }
                TokenValue::Value(_) => {
                    values.push(s);
                }
            }
        }

        for s in symbol.iter() {
            for v in values.iter() {
                if let TokenValue::Value(z) = v.value {
                    if s.y == v.y - 1 || s.y == v.y + 1 || s.y == v.y {
                        if s.startx >= v.startx - 1 && s.startx <= v.endx {
                            sum += z;
                        }
                    }
                }
            }
        }

        Ok(sum)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, AocError> {
        let mut sum = 0;

        let mut values = vec![];
        let mut symbol = vec![];
        for s in input {
            match &s.value {
                TokenValue::Symbol(_) => {
                    symbol.push(s);
                }
                TokenValue::Value(_) => {
                    values.push(s);
                }
            }
        }

        for s in symbol.iter() {
            if let TokenValue::Symbol('*') = s.value {
                let mut ne = vec![];
                for v in values.iter() {
                    if let TokenValue::Value(z) = v.value {
                        if s.y == v.y - 1 || s.y == v.y + 1 || s.y == v.y {
                            if s.startx >= v.startx - 1 && s.startx <= v.endx {
                                ne.push(z)
                            }
                        }
                    }
                }

                if ne.len() == 2 {
                    sum += ne[0] * ne[1];
                }
            }
        }

        Ok(sum)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::solutions::Solver;

    #[test]
    fn a() {
        let i = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let a = Solution::solve(i, true).unwrap();
        dbg!(a);
    }
}
