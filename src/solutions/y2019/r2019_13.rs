use failure::Error;
use crate::solutions::Solver;
use crate::solutions::intcode::{Program, State};
use crate::solutions::common::Vector2;
use std::collections::HashMap;
use itertools::{Itertools, MinMaxResult};
use std::io::{stdin, Write, Read};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io;
use termion::{clear, cursor};
use termion::event::Key;
use std::process::exit;

pub enum Solution {}

impl Solution {

}


#[derive(Eq, PartialEq, Clone, Copy)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4
}

impl From<i64> for Tile {
    fn from(v: i64) -> Self {
        match v {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => unimplemented!()
        }
    }
}

struct Cabinet<W: Write, R> {
    state: MemoryState,
    save_state: Option<MemoryState>,
    stdout: W,
    stdin: R,
    score: i64
}

enum OutputState {
    X,
    Y,
    Data
}

#[derive(Clone)]
struct MemoryState {
    code: Program,
    map: HashMap<Vector2, Tile>
}

impl<W: Write, R: Iterator<Item=Result<Key, std::io::Error>>> Cabinet<W,R> {
    fn new(code :Program, stdin: R, stdout: W) -> Self {
        Cabinet {
            state: MemoryState {
                map: HashMap::new(),
                code
            },
            save_state: None,
            stdout,
            stdin,
            score: 0
        }
    }

    fn save(&mut self) {
        self.save_state = Some(self.state.clone());
    }

    fn load(&mut self) {
        if self.save_state.is_some() {
            self.state = self.save_state.as_ref().unwrap().clone();
            writeln!(self.stdout,
                     "{}{}{}{}",
                     termion::clear::All,
                     termion::cursor::Goto(1, 1),
                     termion::cursor::Hide,
                     termion::style::Reset).unwrap();
            self.print_map();
        }
    }

    fn set_coins(&mut self) {
        self.state.code.code[0] = 2;
    }

    fn get_ball_pos(&self) -> i32 {
        self.state.map.iter().find(|p|*p.1 == Tile::Ball).unwrap().0.x
    }

    fn get_paddle_pos(&self) -> i32 {
        self.state.map.iter().find(|p|*p.1 == Tile::Paddle).unwrap().0.x
    }

    fn handle_input(&mut self){
        loop {
            let b = self.stdin.next().unwrap().unwrap();
            match b {
                Key::Left => {
                    self.state.code.set_input(-1);
                    break;
                },
                Key::Right => {
                    self.state.code.set_input(1);
                    break;
                },
                Key::Char('a') => {
                    self.save();
                    writeln!(self.stdout,
                             "{} state saved",
                             termion::cursor::Goto(1, 1)).unwrap();
                    self.stdout.flush().unwrap();
                },
                Key::Esc => {
                    exit(0);
                }
                Key::Char(' ') => {
                    self.load();
                    writeln!(self.stdout,
                             "{} state loaded",
                             termion::cursor::Goto(1, 1)).unwrap();
                    self.stdout.flush().unwrap();
                },
                Key::Down => {
                    self.state.code.set_input(0);
                    break;
                },
                Key::Char('t') => {
                    let i = i32::signum(self.get_ball_pos() - self.get_paddle_pos());
                    self.state.code.set_input(i as i64);
                    break;
                }
                _ => {}
            };
        }

    }

    fn run(&mut self) -> (HashMap<Vector2, Tile>, i64){
        let mut state = OutputState::X;
        let mut x = 0;
        let mut y = 0;
        let mut tile = Tile::Empty;
        let mut is_score = false;
        writeln!(self.stdout,
                 "{}{}{}{}",
                 termion::clear::All,
                 termion::cursor::Goto(1, 1),
                 termion::cursor::Hide,
                 termion::style::Reset)
            .unwrap();
        self.stdout.flush().unwrap();
        loop {
            match self.state.code.run() {
                State::Input => {
                    self.print_map();
                    let i = i32::signum(self.get_ball_pos() - self.get_paddle_pos());
                    self.state.code.set_input(i as i64);
                },
                State::Output => {
                    let value = self.state.code.consume_output();
                    match state {
                        OutputState::X => {
                            x=value as i32;
                            state = OutputState::Y;
                            if x < 0 {
                                is_score = true
                            }
                        },
                        OutputState::Y => {
                            y = value as i32;
                            state = OutputState::Data;
                        },
                        OutputState::Data => {
                            if is_score {
                                self.score = value;
                             } else {
                                tile = Tile::from(value);
                                self.state.map.insert(Vector2::new(x,y), tile);
                            }
                            state = OutputState::X;
                            is_score = false
                        },
                    }
                },
                State::Finished => {
                    self.print_map();
                    writeln!(self.stdout,
                             "{}Game over",
                             termion::cursor::Goto(1, 2)).unwrap();
                    self.stdout.flush().unwrap();
                    self.handle_input();
                },
                _ => {},
            }
        }
        self.print_map();
        write!(self.stdout, "{}", termion::cursor::Show).unwrap();
        (self.state.map.clone(), self.score)
    }

    fn print_map(&mut self) {
        let (min_width, max_width) = match self.state.map.iter().minmax_by_key(|(k,v)|k.x) {
            MinMaxResult::NoElements => {return},
            MinMaxResult::OneElement(_) => {return},
            MinMaxResult::MinMax(a, b) => {
                (a.0.x, b.0.x)
            },
        };
        let (min_height, max_height) = match self.state.map.iter().minmax_by_key(|(k,v)|k.y) {
            MinMaxResult::NoElements => {return},
            MinMaxResult::OneElement(_) => {return},
            MinMaxResult::MinMax(a, b) => {
                (a.0.y, b.0.y)
            },
        };
        write!(self.stdout, "{}", cursor::Goto(1, 1)).unwrap();
        write!(self.stdout, "score: {}       \n\r", self.score).unwrap();
        for y in min_height..=max_height {
            for x in min_width..=max_width {
                let c = match self.state.map.get(&Vector2::new(x,y)).unwrap_or(&Tile::Empty) {
                    Tile::Empty => {" "},
                    Tile::Wall => {"W"},
                    Tile::Block => {"B"},
                    Tile::Paddle => {"P"},
                    Tile::Ball => {"O"},
                };
                write!(self.stdout, "{}", c).unwrap();
            }
            self.stdout.write(b"\n\r").unwrap();
        }
        self.stdout.flush().unwrap();
    }
}

impl Solver for Solution {
    type Input = Program;
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, Error> {
        Program::parse(input)
    }

    fn solve_part1(mut input: Self::Input) -> Result<Self::Output, Error> {
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        let stdin = io::stdin();
        let stdin = stdin.lock();
        let stderr = io::stderr();
        let mut stderr = stderr.lock();
        let mut c = Cabinet::new(input, stdin.keys(), stdout);
        let (tiles,_) = c.run();
        let res = tiles.iter().filter(|(k,v)| {
            **v == Tile::Block
        }).count();
        Ok(res as i32)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, Error> {
        let stdout = io::stdout();
        let mut stdout = stdout;
        let stdin = io::stdin();
        let stdin = stdin.lock();
        let mut stdout = stdout.into_raw_mode().unwrap();
        let mut c = Cabinet::new(input, stdin.keys(), stdout);
        c.set_coins();
        let (_,res) = c.run();
        Ok(res as i32)
    }
}

#[cfg(test)]
mod tests {

}