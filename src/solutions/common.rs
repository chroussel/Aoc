use itertools::{Itertools, MinMaxResult};
use nom::lib::std::collections::HashMap;
use std::{fmt::Display, io::Write};
use termion::cursor;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vector3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vector3 {
    pub fn new(x: i32, y: i32, z: i32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn zero() -> Vector3 {
        Vector3::new(0, 0, 0)
    }

    pub fn add(&self, other: Vector3) -> Vector3 {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

impl Vector2 {
    pub fn new(x: i32, y: i32) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn zero() -> Vector2 {
        Vector2::new(0, 0)
    }

    pub fn add(&self, other: &Vector2) -> Vector2 {
        Vector2::new(self.x + other.x, self.y + other.y)
    }

    pub fn scale(&self, scalar: i32) -> Vector2 {
        Vector2::new(self.x * scalar, self.y * scalar)
    }

    pub fn neigh(&self) -> Vec<Vector2> {
        let v = vec![
            Vector2::new(1, 0),
            Vector2::new(-1, 0),
            Vector2::new(0, 1),
            Vector2::new(0, -1),
        ];
        v.into_iter().map(|vs| vs.add(self)).collect()
    }
}

pub trait Cell {
    fn default() -> Self;
    fn print<W: Write>(&self, stdout: &mut W);
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    None,
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn to_input(&self) -> i64 {
        match self {
            Direction::None => 0,
            Direction::East => 4,
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }

    pub fn to_vector(&self) -> Vector2 {
        match self {
            Direction::None => Vector2::zero(),
            Direction::North => Vector2::new(0, -1),
            Direction::South => Vector2::new(0, 1),
            Direction::East => Vector2::new(1, 0),
            Direction::West => Vector2::new(-1, 0),
        }
    }

    pub fn rotate(&self) -> Self {
        match self {
            Direction::None => Direction::North,
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct Map<T>(pub Vec<Vec<T>>);

impl<T> Map<T> {
    pub fn iter_row(
        &self,
        col: usize,
    ) -> impl Iterator<Item = &T> + DoubleEndedIterator + ExactSizeIterator {
        self.0[col].iter()
    }

    pub fn iter_col<'a>(
        &'a self,
        row: usize,
    ) -> impl Iterator<Item = &'a T> + DoubleEndedIterator + ExactSizeIterator {
        self.0.iter().map(move |r| &r[row])
    }

    pub fn shape(&self) -> (usize, usize) {
        (
            self.0.len(),
            self.0.get(0).map(|r| r.len()).unwrap_or_default(),
        )
    }
}
impl<T: Display> Map<T> {
    pub fn print_map(&self) {
        let (rows, cols) = self.shape();
        println!("----");
        for i in 0..rows {
            for j in 0..cols {
                print!("{}", self.0[i][j]);
            }
            println!("");
        }
        println!("----");
    }
}
#[derive(Clone)]
pub struct Map2D<T: Cell, W: Write> {
    pub data: HashMap<Vector2, T>,
    stdout: W,
}

impl<T: Cell, W: Write> Map2D<T, W> {
    pub fn new(stdout: W) -> Self {
        Map2D {
            data: HashMap::new(),
            stdout,
        }
    }

    pub fn init(&mut self) {
        writeln!(
            self.stdout,
            "{}{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide,
            termion::style::Reset
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn print_map(&mut self, player: Vector2, symbol: &T, clean: bool) {
        let (min_width, max_width) = match self.data.iter().minmax_by_key(|(k, _v)| k.x) {
            MinMaxResult::NoElements => return,
            MinMaxResult::OneElement(_) => return,
            MinMaxResult::MinMax(a, b) => (a.0.x, b.0.x),
        };
        let (min_height, max_height) = match self.data.iter().minmax_by_key(|(k, _v)| k.y) {
            MinMaxResult::NoElements => return,
            MinMaxResult::OneElement(_) => return,
            MinMaxResult::MinMax(a, b) => (a.0.y, b.0.y),
        };

        if clean {
            write!(self.stdout, "{}", cursor::Goto(1, 1)).unwrap();
        }
        for y in min_height..=max_height {
            for x in min_width..=max_width {
                let default = T::default();
                let current = Vector2::new(x, y);
                let c = if current == player {
                    symbol
                } else {
                    self.data.get(&current).unwrap_or(&default)
                };
                c.print(&mut self.stdout);
            }
            self.stdout.write(b"\n\r").unwrap();
        }
        self.stdout.flush().unwrap();
    }
}

impl<T: Cell, W: Write> Drop for Map2D<T, W> {
    fn drop(&mut self) {
        writeln!(
            self.stdout,
            "{}{}",
            termion::cursor::Show,
            termion::style::Reset
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }
}
