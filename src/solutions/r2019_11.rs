use failure::Error;
use crate::solutions::Solver;
use crate::solutions::intcode::{Program, State};
use std::collections::HashSet;
use crate::solutions::common::Vector2;

pub enum Solution {}

pub struct Panel {
    is_white: bool,
}


pub struct Robot {
    program: Program,
    white_panel: HashSet<Vector2>,
    black_panel: HashSet<Vector2>
}

pub enum RobotState {
    Color,
    Move
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn rot_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn rot_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn to_vector(&self) -> Vector2 {
        match self {
            Direction::Up => Vector2::new(0, 1),
            Direction::Down => Vector2::new(0, -1),
            Direction::Left => Vector2::new(1, 0),
            Direction::Right => Vector2::new(-1, 0),
        }
    }
}

impl Robot {
    fn new(program: Program, start_color: bool) -> Robot {
        let mut white_panel: HashSet<Vector2> = HashSet::new();
        let mut black_panel: HashSet<Vector2> = HashSet::new();
        if start_color {
            white_panel.insert(Vector2::new(0,0));
        }
        Robot {
            program,
            white_panel,
            black_panel
        }
    }

    fn run(&mut self) {
        let mut position = Vector2::new(0,0);
        let mut direction = Direction::Up;
        let mut state = RobotState::Color;
        loop {
            match self.program.run() {
                State::Input => {
                    let input = if self.white_panel.contains(&position) {
                        1
                    } else {
                        0
                    };
                    self.program.set_input(input);
                },
                State::Output => {
                    match state {
                        RobotState::Color => {
                            match self.program.consume_output() {
                                0 => {
                                    self.white_panel.remove(&position);
                                    self.black_panel.insert(position);
                                },
                                1 => {
                                    self.black_panel.remove(&position);
                                    self.white_panel.insert(position);
                                },
                                _=> unimplemented!()
                            }
                            state = RobotState::Move
                        },
                        RobotState::Move => {
                            match self.program.consume_output() {
                                0 => {direction = direction.rot_left();},
                                1 => {direction = direction.rot_right();},
                                _=> unimplemented!()
                            }
                            position = position.add(&direction.to_vector());
                            state = RobotState::Color;
                        },
                    }
                },
                State::Finished => {
                    break
                },
                _ => {}
            }
        }
    }

    fn paint_count(&self) -> i32 {
        self.black_panel.union(&self.white_panel).count() as i32
    }

    fn print(&self) {
        let min_width = self.white_panel.iter().map(|p|p.x).min().unwrap();
        let max_width = self.white_panel.iter().map(|p|p.x).max().unwrap();
        let min_height = self.white_panel.iter().map(|p|p.y).min().unwrap();
        let max_height = self.white_panel.iter().map(|p|p.y).max().unwrap();
        dbg!(min_width, max_width);
        dbg!(min_height, max_height);

        for y in (min_height..=max_height).rev() {
            for x in (min_width..=max_width).rev() {
                let p = Vector2::new(x,y);
                if self.white_panel.contains(&p) {
                    print!("#")
                } else {
                    print!(" ")
                }
            }
            println!()
        }
    }
}

impl Solver for Solution {
    type Input = Program;
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, Error> {
        Program::parse(input)
    }

    fn solve_part1(mut input: Self::Input) -> Result<Self::Output, Error> {
        let mut res = Robot::new(input, false);
        res.run();
        Ok(res.paint_count())
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, Error> {
        let mut res = Robot::new(input, true);
        res.run();
        res.print();
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::r2019_11::Solution;
    use crate::solutions::Solver;
    use crate::solutions::intcode::Program;

    #[test]
    fn e1() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let res = Program::parse(input).run_with_input(vec!());
        dbg!(res);
    }

    #[test]
    fn e2() {
        let input = "104,1125899906842624,99";
        let res = Program::parse(input).run_with_input(vec!());
        dbg!(res);
    }

    #[test]
    fn e3() {
        let input = "1102,34915192,34915192,7,4,7,99,0";
        let res = Program::parse(input).run_with_input(vec!());
        dbg!(res);
    }
}