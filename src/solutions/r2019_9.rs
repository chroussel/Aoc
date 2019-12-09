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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum ParamMode {
    Position = 0,
    Immediate = 1,
    Relative = 2
}

impl From<i64> for ParamMode {
    fn from(value: i64) -> Self {
        match value {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            _ => unimplemented!()
        }
    }
}

struct Program {
    code: Vec<i64>,
    pc: usize,
    state: State,
    relative_base: i64,
    input: Option<i64>,
    output: Option<i64>
}

impl Program {
    fn new(code: Vec<i64>) -> Self {
        Program {
            code,
            pc: 0,
            relative_base: 0,
            state:State::Start,
            input:None,
            output:None
        }
    }

    fn set_input(&mut self, input: i64) {
        self.input = Some(input);
    }

    fn consume_output(&mut self) -> i64 {
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

    fn get_value(&self, value: i64, param: ParamMode) -> i64 {
        match param {
            ParamMode::Position => {
                self.code.get(value as usize).map(|&a|a).unwrap_or(0)
            },
            ParamMode::Immediate => {
                value
            },
            ParamMode::Relative => {
                self.code.get((self.relative_base + value) as usize)
                    .map(|&a|a)
                    .unwrap_or(0)
            },
        }
    }

    fn set_value(&mut self, destination: i64, value: i64, param: ParamMode) {
        let address = match param {
            ParamMode::Position => {
                destination
            },
            ParamMode::Immediate => {
                unimplemented!()
            },
            ParamMode::Relative => {
                (self.relative_base + destination)
            },
        } as usize;
        if address >= self.code.len() {
            self.code.resize(address + 1, 0);
        }
        self.code[address as usize] = value;
    }


    fn step(&mut self) -> State {
        self.state = State::Running;
        let op_code = self.code[self.pc];
        let param1 = From::from(op_code % 1000 / 100);
        let param2 = From::from(op_code % 10000 / 1000);
        let param3: ParamMode =  From::from(op_code % 100000 / 10000);
        //dbg!(op_code);
        //dbg!(param1, param2);
        match op_code % 100 {
            1 => {
                let v1 = self.get_value(self.code[self.pc+1], param1);
                let v2 = self.get_value(self.code[self.pc+2], param2);
                self.set_value(self.code[self.pc+3], v1 + v2, param3);
                self.pc+=4;
            },
            2 => {
                let v1 = self.get_value(self.code[self.pc+1], param1);
                let v2 = self.get_value( self.code[self.pc+2], param2);
                self.set_value(self.code[self.pc+3], v1 * v2, param3);
                self.pc+=4;
            },
            3 => {
                if self.input.is_none() {
                    self.state = State::Input;
                    return State::Input;
                } else {
                    let value = self.input.take().unwrap();
                    self.set_value(self.code[self.pc+1], value, param1);
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
                let value = if v1 < v2 {
                    1
                } else {
                   0
                };
                self.set_value(self.code[self.pc+3], value, param3);
                self.pc+=4;
            }
            8 => {
                let v1 = self.get_value( self.code[self.pc+1], param1);
                let v2 = self.get_value( self.code[self.pc+2], param2);
                let value = if v1 == v2 {
                    1
                } else {
                    0
                };
                self.set_value(self.code[self.pc+3], value, param3);
                self.pc+=4;
            }
            9 => {
                let v1 = self.get_value( self.code[self.pc+1], param1);
                self.relative_base += v1;
                self.pc+=2;
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
    pub fn run(code: &[i64], mut input: Vec<i64>) -> Vec<i64>{
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
}

impl Solver for Solution {
    type Input = Vec<i64>;
    type Output = i64;

    fn parse_input(input: &str) -> Result<Self::Input, Error> {
        input.trim_end()
            .split(',')
            .filter(|s|!s.is_empty())
            .map(|u|u.parse().map_err(From::from))
            .collect()
    }

    fn solve_part1(mut input: Self::Input) -> Result<Self::Output, Error> {
        let res = Solution::run(&input, vec!(1));
        dbg!(&res);
        Ok(*res.last().unwrap())
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, Error> {
        let res = Solution::run(&input, vec!(2));
        dbg!(&res);
        Ok(*res.last().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::r2019_9::Solution;
    use crate::solutions::Solver;

    #[test]
    fn e1() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut code = Solution::parse_input(input).unwrap();
        let res = Solution::run(&code, vec!());
        dbg!(res);
    }

    #[test]
    fn e2() {
        let input = "104,1125899906842624,99";
        let mut code = Solution::parse_input(input).unwrap();
        code.resize(200, 0);
        let res = Solution::run(&code, vec!());
        dbg!(res);
    }

    #[test]
    fn e3() {
        let input = "1102,34915192,34915192,7,4,7,99,0";
        let mut code = Solution::parse_input(input).unwrap();
        code.resize(200, 0);
        let res = Solution::run(&code, vec!());
        dbg!(res);
    }
}