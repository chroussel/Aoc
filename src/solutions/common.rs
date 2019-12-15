use nom::lib::std::collections::HashMap;
use itertools::{MinMaxResult, Itertools};
use termion::cursor;
use std::io::Write;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vector3 {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Vector3 {
    pub fn new(x: i32, y: i32, z: i32) -> Vector3 {
        Vector3 { x,y,z}
    }

    pub fn zero() -> Vector3 {
        Vector3::new(0,0,0)
    }

    pub fn add(&self, other: Vector3) -> Vector3 {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}


#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32
}

impl Vector2 {
    pub fn new(x: i32, y: i32) -> Vector2 {
        Vector2 {x,y}
    }

    pub fn zero() -> Vector2 {
        Vector2::new(0,0)
    }

    pub fn add(&self, other: &Vector2) -> Vector2 {
        Vector2::new(self.x + other.x, self.y + other.y)
    }

    pub fn scale(&self, scalar: i32) -> Vector2 {
        Vector2::new(self.x * scalar, self.y * scalar)
    }
}

pub trait Cell {
    fn default() -> Self;
    fn print(&self) -> &str;
}

#[derive(Clone)]
pub struct Map2D<T: Cell, W: Write> {
    pub data: HashMap<Vector2, T>,
    stdout: W
}

impl<T: Cell, W: Write> Map2D<T, W> {
    pub fn new(stdout: W) -> Self {
        Map2D {
            data: HashMap::new(),
            stdout
        }
    }

    pub fn init(&mut self) {
        writeln!(self.stdout,
                 "{}{}{}{}",
                 termion::clear::All,
                 termion::cursor::Goto(1, 1),
                 termion::cursor::Hide,
                 termion::style::Reset).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn print_map(&mut self, player: Vector2, symbol: &T) {
        let (min_width, max_width) = match self.data.iter().minmax_by_key(|(k,v)|k.x) {
            MinMaxResult::NoElements => {return},
            MinMaxResult::OneElement(_) => {return},
            MinMaxResult::MinMax(a, b) => {
                (a.0.x, b.0.x)
            },
        };
        let (min_height, max_height) = match self.data.iter().minmax_by_key(|(k,v)|k.y) {
            MinMaxResult::NoElements => {return},
            MinMaxResult::OneElement(_) => {return},
            MinMaxResult::MinMax(a, b) => {
                (a.0.y, b.0.y)
            },
        };
        //write!(self.stdout, "{}", cursor::Goto(1, 1)).unwrap();
        for y in min_height..=max_height {
            for x in min_width..=max_width {
                let default = T::default();
                let current = Vector2::new(x,y);
                let c = if current == player {
                    symbol
                } else {
                    self.data.get(&current).unwrap_or(&default)
                };
                write!(self.stdout, "{}", c.print()).unwrap();
            }
            self.stdout.write(b"\n\r").unwrap();
        }
        self.stdout.flush().unwrap();
    }
}

impl<T: Cell, W: Write> Drop for Map2D<T, W> {
    fn drop(&mut self) {
        writeln!(self.stdout,
                 "{}{}{}{}",
                 termion::clear::All,
                 termion::cursor::Goto(1, 1),
                 termion::cursor::Hide,
                 termion::style::Reset).unwrap();
        self.stdout.flush().unwrap();
    }
}