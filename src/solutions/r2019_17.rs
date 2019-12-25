use failure::Error;
use crate::solutions::Solver;
use crate::solutions::intcode::{Program, State};
use crate::solutions::common::{Map2D, Cell, Direction, Vector2};
use std::io::Write;
use std::io;
use termion::raw::IntoRawMode;
use termion::input::TermRead;

pub enum Solution {}

impl Solution {
}

#[derive(Copy, Clone, Debug)]
enum Tile {
    Ascii(u8),
}

impl Tile {
    fn from_ascii(c: u8) -> Tile{
        Tile::Ascii(c)
    }

    fn is_scalfold(&self) -> bool {
        if let Tile::Ascii(a) = self {
            if *a == b'#' {
                return true
            }
        }
        return false
    }
}

impl Cell for Tile {
    fn default() -> Self {
        Tile::Ascii(32)
    }

    fn print<W: Write>(&self, stdout: &mut W) {
        match self {
            Tile::Ascii(c) => {
                stdout.write(&[*c]);
            },
        }
    }
}

struct Robot<W: Write> {
    p: Program,
    map: Map2D<Tile, W>,
    path: Vec<u8>
}

impl<W: Write> Robot<W> {
    fn new(p: Program, stdout: W) -> Robot<W> {
        Robot {
            p,
            map: Map2D::new(stdout),
            path: vec![]
        }
    }

    fn set_path(&mut self, path: &str) {
        self.path.extend_from_slice(path.as_bytes());
        self.path.reverse();
    }

    fn run(&mut self) -> i64 {
        self.map.init();
        let mut camera_pos = Vector2::zero();
        let mut output = 0;
        let mut modu = 50;
        loop {

            match self.p.run() {
                State::Input => {
                    let i = self.path.pop().unwrap();
                    self.p.set_input(i as i64);
                    if self.path.is_empty() {
                        modu = 40;
                        camera_pos.y = -1;
                    }
                },
                State::Output => {
                    let o = self.p.consume_output();
                    let mut tile = Tile::default();
                    match o {
                        10 => {
                            camera_pos = Vector2::new(0, (camera_pos.y + 1) % modu);
                            self.map.print_map(Vector2::zero(), &Tile::Ascii('X' as u8), true);
                            continue;
                        },
                        i if i > 256 => {
                            output = i;
                            continue
                        }
                        _ => {
                            tile = Tile::Ascii(o as u8);
                        }
                    }
                    self.map.data.insert(camera_pos, tile);
                    camera_pos.x += 1;
                },
                State::Finished => { break },
                _ => {}
            }
        }
        self.map.print_map(Vector2::zero(), &Tile::Ascii('X' as u8), true);
        output
    }
}

impl Solver for Solution {
    type Input = Program;
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, Error> {
        Program::parse(input)
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, Error> {
        let mut r = Robot::new(input, io::stdout());
        r.run();
        let align = r.map.data.iter().filter(|(pos, t)| {
            t.is_scalfold() && pos.neigh().iter().all(|p| {
                if let Some(t) = r.map.data.get(p) {
                    return t.is_scalfold();
                }
                return false
            })
        }).map(|(pos, _)| {
            pos.x * pos.y
        }).sum();
        Ok(align)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, Error> {
        let a = b"R12L8L4L4L8R6L6\n";
        let b = b"R6R6L8R4R4R8R12\n";
        let c = b"L6R6L6R6\n";
        let d = b"R12L4L6R12L4\n";
        let e = b"R8R4R4R8\n";
        let f = b"R12L6L4R12L8L4L4L8L4\n";
        let g = b"R12L6L4L8R6L6\n";
        let input_t = b"AARBCDEDRR4FFG";
        let stderr = io::stderr();
        let stderr = stderr.lock();
        let mut stdout = io::stdout();
        let stdout = stdout.lock();
        let stdout = stdout.into_raw_mode().unwrap();

        let mut r = Robot::new(input, stdout);
        r.p.code[0] = 2;
        r.set_path("A,B,A,B,C,A,C,A,C,B\nR,12,L,8,L,4,L,4\nL,8,R,6,L,6\nL,8,L,4,R,12,L,6,L,4\ny\n");
        let result = r.run();
        let complete = "R,12,L,8,L,4,L,4,L,8,R,6,L,6,R,12,L,8,L,4,L,4,L,8,R,6,L,6,L,8,L,4,R,12,L,6,L,4,R,12,L,8,L,4,L,4,L,8,L,4,R,12,L,6,L,4,R,12,L,8,L,4,L,4,L,8,L,4,R,12,L,6,L,4,R,8,L,6,R,6";
        Ok(result as i32)

    }
}

#[cfg(test)]
mod tests {

}