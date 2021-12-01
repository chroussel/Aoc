use failure::Error;
use crate::solutions::Solver;
use std::ops::Range;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum State {
    Start,
    Input,
    Output,
    Running,
    Finished
}

struct Program {
    code: Vec<i32>,
    pc: usize,
    state: State,
    input: Option<i32>,
    output: Option<i32>
}

impl Program {
    fn new(code: Vec<i32>) -> Self {
        Program {
            code,
            pc: 0,
            state:State::Start,
            input:None,
            output:None
        }
    }

    fn set_input(&mut self, input: i32) {
        self.input = Some(input);
    }

    fn consume_output(&mut self) -> i32 {
        self.pc+=2;
        self.output.take().unwrap()
    }

    fn is_complete(&self) -> bool {
        self.state == State::Finished
    }

    fn state(&self) -> State {
        self.state
    }

    fn run(&mut self) -> State {
        if self.is_complete() {
            return State::Finished;
        }
        loop {
            let step = self.step();
            match step {
                State::Input => {
                    return State::Input;
                },
                State::Output => {
                    return State::Output;
                },
                State::Finished => {
                    return State::Finished;
                },
                _ => {}
            }
        }
    }

    fn get_value(&self, value: i32, param: bool) -> i32 {
        if param {
            value
        } else {
            self.code[value as usize]
        }
    }

    fn step(&mut self) -> State {
        self.state = State::Running;
        let op_code = self.code[self.pc];
        let param1 = op_code % 1000 / 100 == 1;
        let param2 = op_code % 10000 / 1000 == 1;
        let param3 =  op_code % 100000 / 10000 == 1;
        //dbg!(op_code);
        match op_code % 100 {
            1 => {
                let v1 = self.get_value(self.code[self.pc+1], param1);
                let v2 = self.get_value(self.code[self.pc+2], param2);
                let oi= self.code[self.pc+3] as usize;
                self.pc+=4;
                self.code[oi as usize] = v1 + v2;
            },
            2 => {
                let v1 = self.get_value(self.code[self.pc+1], param1);
                let v2 = self.get_value( self.code[self.pc+2], param2);
                let oi= self.code[self.pc+3] as usize;
                self.pc+=4;
                self.code[oi as usize] = v1 * v2;
            },
            3 => {
                if self.input.is_none() {
                    self.state = State::Input;
                    return State::Input;
                } else {
                    let address = self.code[self.pc+1] as usize;
                    self.code[address] = self.input.take().unwrap();
                    self.pc+=2;
                    self.state = State::Running;
                }
            },
            4 => {
                if self.output.is_none() {
                    let value = self.get_value( self.code[self.pc+1], param1);
                    self.output = Some(value);
                }
                self.state = State::Output;
                return State::Output;
            }
            5 => {
                let v1 = self.get_value(self.code[self.pc+1], param1);
                if v1 > 0 {
                    self.pc = self.get_value(self.code[self.pc+2], param2) as usize;
                } else {
                    self.pc+=3;
                }

            }
            6 => {
                let v1 = self.get_value(self.code[self.pc+1], param1);
                if v1 == 0 {
                    self.pc = self.get_value( self.code[self.pc+2], param2) as usize;
                } else {
                    self.pc+=3;
                }
            }
            7 => {
                let v1 = self.get_value( self.code[self.pc+1], param1);
                let v2 = self.get_value( self.code[self.pc+2], param2);
                let v3 = self.code[self.pc+3] as usize;
                self.pc+=4;
                if v1 < v2 {
                    self.code[v3] = 1;
                } else {
                    self.code[v3] = 0;
                }
            }
            8 => {
                let v1 = self.get_value( self.code[self.pc+1], param1);
                let v2 = self.get_value( self.code[self.pc+2], param2);
                let v3 = self.code[self.pc+3] as usize;
                self.pc+=4;
                if v1 == v2 {
                    self.code[v3] = 1;
                } else {
                    self.code[v3] = 0;
                }
            }
            99 => {
                self.state = State::Finished;
            }
            _ => {unimplemented!()}
        }

        return self.state;
    }
}

pub enum Solution {}


impl Solution {
    pub(crate) fn run(code: &[i32], mut input: Vec<i32>) -> Vec<i32>{
        let mut code = code.to_vec();
        let mut p = Program::new(code);
        let mut output = vec![];
        loop {
            match p.run() {
                State::Input => {
                    p.set_input(input.pop().unwrap());
                },
                State::Output => {
                    output.push(p.consume_output())
                },
                State::Finished => {
                    break;
                },
                _ => {}
            }
        }
        output
    }

    fn run_seq(code: &[i32], mut seq: Vec<i32>) -> i32{
        let mut res = 0;
        while let Some(s) = seq.pop() {
            let o = Solution::run(code, vec![res, s]);
            res = o[0];
        }
        res
    }

    fn run_seqv2(code: &[i32], mut seq: Vec<i32>) -> i32{
        let mut ps:Vec<Program> = seq.into_iter()
            .map(|s| {
                let mut p = Program::new(code.to_vec());
                p.set_input(s);
                p
            })
            .collect();

        let mut cp = 0;
        let mut signal = 0;
        loop {
            let curp = &mut ps[cp];
            curp.run();
            if curp.state == State::Input {
                curp.set_input(signal);
            }
            match curp.run() {
                State::Output => {
                    signal = curp.consume_output();
                },
                State::Finished => {
                    return signal;
                },
                _ => {}
            }
            cp = (cp + 1)%ps.len();
        }
    }


    fn all_permutation(v: Vec<i32>) -> Vec<Vec<i32>> {
        Solution::all_permutation_inner(v.clone(), 0..v.len())
    }

    fn all_permutation_inner(mut v: Vec<i32>, range: Range<usize>) -> Vec<Vec<i32>> {
        if range.len() <= 0 {
            return vec![v];
        }
        let mut res = vec![];
        for i in range.clone() {
            v.swap(range.start, i);
            let mut inner_res = Solution::all_permutation_inner(v.clone(), range.start+1..range.end);
            res.append(&mut inner_res);
            v.swap(range.start, i);
        }
        res
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

    fn solve_part1(input: Self::Input) -> Result<Self::Output, Error> {
        let mut seq = vec!(1,0,4,3,2);
        let res = Solution::all_permutation(seq)
            .into_iter()
            .map(|v| Solution::run_seq(&input, v))
            .max();
        Ok(res.unwrap())
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, Error> {
        let mut seq = vec!(5,6,7,8,9);
        let res = Solution::all_permutation(seq)
            .into_iter()
            .map(|v| Solution::run_seqv2(&input, v))
            .max();
        Ok(res.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::r2019_7::Solution;
    use crate::solutions::Solver;

    #[test]
    fn e1() {
        let code = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        //let res = 10 * b + a;
        let res = Solution::solve(code, true);
        dbg!(res);
    }

    #[test]
    fn e2() {
        let code = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let res = Solution::solve(code, true);
        dbg!(res);
    }

    #[test]
    fn e3() {
        let code = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let res = Solution::solve(code, true);
        dbg!(res);
    }
}