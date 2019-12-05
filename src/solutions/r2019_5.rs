use failure::Error;
use crate::solutions::Solver;

pub enum Solution {}

impl Solution {
    fn get_value(input: &[i32], value: i32, param: bool) -> i32 {
        if param {
            value
        } else {
            input[value as usize]
        }
    }

    fn run(code: &[i32], input: i32) -> i32{
        let mut code = code.to_vec();
        let mut pc = 0;
        loop {
            let op_code = code[pc];
            let param1 = op_code % 1000 / 100 == 1;
            let param2 = op_code % 10000 / 1000 == 1;
            let param3 =  op_code % 100000 / 10000 == 1;
            dbg!(op_code);
            match op_code % 100 {
                1 => {
                    let v1 = Solution::get_value(&code, code[pc+1], param1);
                    let v2 = Solution::get_value(&code, code[pc+2], param2);
                    let oi= code[pc+3] as usize;
                    pc+=4;
                    code[oi as usize] = v1 + v2;
                },
                2 => {
                    let v1 = Solution::get_value(&code, code[pc+1], param1);
                    let v2 = Solution::get_value(&code, code[pc+2], param2);
                    let oi= code[pc+3] as usize;
                    pc+=4;
                    code[oi as usize] = v1 * v2;
                },
                3 => {
                    let address = code[pc+1] as usize;
                    code[address] = input;
                    pc+=2;
                },
                4 => {
                    let value = Solution::get_value(&code, code[pc+1], param1);
                    println!("{}", value);
                    pc+=2;
                }
                5 => {
                    let v1 = Solution::get_value(&code, code[pc+1], param1);
                    if v1 > 0 {
                        pc = Solution::get_value(&code, code[pc+2], param2) as usize;
                    } else {
                        pc+=3;
                    }

                }
                6 => {
                    let v1 = Solution::get_value(&code, code[pc+1], param1);
                    if v1 == 0 {
                        pc = Solution::get_value(&code, code[pc+2], param2) as usize;
                    } else {
                        pc+=3;
                    }
                }
                7 => {
                    let v1 = Solution::get_value(&code, code[pc+1], param1);
                    let v2 = Solution::get_value(&code, code[pc+2], param2);
                    let v3 = code[pc+3] as usize;
                    pc+=4;
                    if v1 < v2 {
                        code[v3] = 1;
                    } else {
                        code[v3] = 0;
                    }
                }
                8 => {
                    let v1 = Solution::get_value(&code, code[pc+1], param1);
                    let v2 = Solution::get_value(&code, code[pc+2], param2);
                    let v3 = code[pc+3] as usize;
                    pc+=4;
                    if v1 == v2 {
                        code[v3] = 1;
                    } else {
                        code[v3] = 0;
                    }
                }
                99 => {
                    break
                }
                _ => {unimplemented!()}
            }
        }
        code[0]
    }
}

impl Solver for Solution {
    type Input = Vec<i32>;
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, Error> {
        input.trim_end()
            .split(',')
            .filter(|s|!s.is_empty())
            .map(|u|u.parse().map_err(From::from))
            .collect()
    }

    fn solve_part1(mut input: Self::Input) -> Result<Self::Output, Error> {
        input.resize(input.len().max(4096), 0);
        Ok(Solution::run(&input, 1))
    }

    fn solve_part2(mut input: Self::Input) -> Result<Self::Output, Error> {
        input.resize(input.len().max(4096), 0);
        Ok(Solution::run(&input, 5))
    }
}