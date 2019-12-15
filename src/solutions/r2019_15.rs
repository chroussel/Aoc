use failure::Error;
use crate::solutions::Solver;
use crate::solutions::intcode::{Program, State};
use crate::solutions::common::{Vector2, Map2D, Cell};
use nom::lib::std::collections::{HashMap, VecDeque, HashSet};
use std::io::Write;
use std::io;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;
use std::process::exit;
use itertools::Itertools;

pub enum Solution {}

impl Solution {

}

#[derive(Debug, Copy, Clone)]
enum Move {
    None,
    North,
    South,
    East,
    West
}

impl Move {
    fn to_input(&self) -> i64 {
        match self {
            Move::None => 0,
            Move::East => 4,
            Move::North => 1,
            Move::South => 2,
            Move::West => 3
        }
    }

    fn to_vector(&self) -> Vector2 {
        match self {
            Move::None => {Vector2::zero()},
            Move::North => {Vector2::new(0,-1)},
            Move::South => {Vector2::new(0,1)},
            Move::East => {Vector2::new(1,0)},
            Move::West => {Vector2::new(-1,0)},
        }
    }

    fn rotate(&self) -> Self {
        match self {
            Move::None => {Move::North},
            Move::North => {Move::East},
            Move::South => {Move::West},
            Move::East => {Move::South},
            Move::West => {Move::North},
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Unknown,
    Empty,
    Wall,
    Droid,
    Oxy
}

impl Cell for Tile {
    fn default() -> Self {
        Tile::Unknown
    }

    fn print(&self) -> &str {
        match self {
            Tile::Unknown => {" "}
            Tile::Empty => {"."},
            Tile::Wall => {"W"},
            Tile::Droid => {"D"},
            Tile::Oxy => {"O"},
        }
    }
}

#[derive(Clone)]
struct Droid<W: Write, R> {
    program: Program,
    map: Map2D<Tile, W>,
    stdin: R,
    position: Vector2
}

enum Utils {}
impl Utils {
    fn neigh(pos: Vector2) -> Vec<Vector2> {
        let v = vec!(Vector2::new(1, 0), Vector2::new(-1, 0), Vector2::new(0, 1), Vector2::new(0, -1));
        v.into_iter().map(|vs| vs.add(&pos)).collect()
    }

    fn get_move(pos: &Vector2, next: &Vector2) -> Move {
        if pos.x > next.x {
            Move::West
        } else if pos.x < next.x {
            Move::East
        } else if pos.y > next.y {
            Move::North
        } else if pos.y < next.y {
            Move::South
        } else {
            Move::None
        }

    }
}

impl<W: Write, R: Iterator<Item=Result<Key, std::io::Error>>> Droid<W, R> {
    fn new(program: Program, stdout: W, stdin: R) -> Self {
        let mut map = Map2D::new(stdout);
        map.data.insert(Vector2::zero(), Tile::Empty);
        Droid {
            program,
            map ,
            stdin,
            position: Vector2::zero()
        }
    }

    fn build_path(&self, start: Vector2, mut target: Vector2, path: HashMap<Vector2, (i32, Vector2)>) -> VecDeque<Move> {
        let mut v = VecDeque::new();
        while let Some(&(_, p)) = path.get(&target) {
            v.push_front(Utils::get_move(&p, &target));
            target = p;
            if p == start {
                break;
            }
        }
        v
    }
    fn get_input(&mut self) -> Move{
        loop {
            let b = self.stdin.next().unwrap().unwrap();
            match b {
                Key::Left => return Move::West,
                Key::Right => return Move::East,
                Key::Up => return Move::North,
                Key::Down => return Move::South,
                Key::Esc => {
                    exit(0);
                }
                _ => {}
            }
        }
    }

    fn get_path(&self, start: Vector2, target: Vector2) -> VecDeque<Move> {
        let mut path = HashMap::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((0, start));
        while let Some((d, q)) = queue.pop_front() {
            if visited.contains(&q) {
                continue;
            }
            if q == target {
                break;
            }
            if let Some(&t) = self.map.data.get(&q) {
                if t != Tile::Empty && t!= Tile::Oxy {
                    continue;
                }
            } else {
                continue;
            }
            visited.insert(q);
            for v in Utils::neigh(q) {
                if visited.contains(&v) {
                    continue;
                }
                queue.push_back((d + 1, v));
                path.entry(v)
                    .and_modify(|(prev_d, source)| {
                        if d + 1 < *prev_d {
                            *prev_d = d + 1;
                            *source = q;
                        }
                    })
                    .or_insert((d + 1, q));
            }
        }
        self.build_path(start, target, path)
    }

    fn run(&mut self) {
        let mut previous_input = Move::North;
        let mut moved = false;
        let mut to_explore = vec!(Vector2::new(1, 0), Vector2::new(-1, 0), Vector2::new(0, 1), Vector2::new(0, -1));
        let mut input_list: VecDeque<Move> = VecDeque::new();
        self.map.init();
        loop {
            match self.program.run() {
                State::Input => {
                    self.map.print_map(self.position, &Tile::Droid);
                    //dbg!(&input_list, &self.position, &self.map.data);
                    loop {
                        if let Some(m) = input_list.pop_front() {
                            previous_input = m;
                            self.program.set_input(previous_input.to_input());
                            break;
                        } else {
                            if let Some(t) = to_explore.pop() {
                                if let Some(a) = self.map.data.get(&t) {
                                    continue
                                }
                                input_list = self.get_path(self.position.clone(), t);
                            } else {
                                return;
                            }
                        }
                    }
                },
                State::Output => {
                    let next_pos = self.position.add(&previous_input.to_vector());
                    match self.program.consume_output() {
                        0 => {
                            self.map.data.insert(next_pos, Tile::Wall);
                        } //wall
                        1 => {
                            self.position = next_pos;
                            self.map.data.insert(self.position, Tile::Empty);
                            for v in Utils::neigh(self.position) {
                                if !self.map.data.contains_key(&v) {
                                    to_explore.push(v);
                                }
                            }

                        } // moved
                        2 => {
                            self.map.data.insert(next_pos, Tile::Oxy);
                            self.position = next_pos;
                            for v in Utils::neigh(self.position) {
                                if !self.map.data.contains_key(&v) {
                                    to_explore.push(v);
                                }
                            }
                        } //oxygen
                        _ => unimplemented!()
                    }
                },
                State::Finished => {
                    break

                },
                _ => {}
            }
        }
    }
}

impl Solver for Solution {
    type Input = Program;
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, Error> {
        Program::parse(input)
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, Error> {
        let stdout = io::stdout();
        //let mut stdout = stdout.into_raw_mode().unwrap();
        let stdin = io::stdin();
        let mut d = Droid::new(input, stdout, stdin.keys());
        d.run();
        let pos = d.map.data.iter().find(|&(v, t)|*t == Tile::Oxy).unwrap();
        let p = d.get_path(Vector2::zero(), *pos.0);
        Ok(p.len() as i32)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, Error> {
        let stdout = io::stdout();
        //let mut stdout = stdout.into_raw_mode().unwrap();
        let stdin = io::stdin();
        let mut d = Droid::new(input, stdout, stdin.keys());
        d.run();
        let (oxy_pos, _) = d.map.data.iter().find(|&(v, t)|*t == Tile::Oxy).unwrap();
        let all_pos = d.map.data.iter()
            .filter(|&(v, t)| *t == Tile::Empty)
            .map(|(v, t)| *v).collect_vec();
        dbg!(all_pos.len());
        let max_dist = all_pos.iter().map(|p| {
            let l = d.get_path(*oxy_pos, *p).len();
            dbg!(l);
            l
        }).max().unwrap();
        Ok(max_dist as i32)
    }
}

#[cfg(test)]
mod tests {

}