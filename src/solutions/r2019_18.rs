use failure::Error;
use crate::solutions::Solver;
use crate::solutions::common::Vector2;
use itertools::Itertools;
use nom::lib::std::collections::{HashMap, VecDeque, HashSet};
use std::fmt::Display;
use failure::_core::fmt::Formatter;
use std::iter::FromIterator;
use rayon::prelude::*;
use std::sync::{Arc, RwLock};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Tile {
    Wall,
    Empty,
    Key(char),
    Door(char),
    Start
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '#' => Tile::Wall,
            '.' => Tile::Empty,
            a if a.is_ascii_lowercase() => Tile::Key(a),
            a if a.is_ascii_uppercase() => Tile::Door(a),
            '@' => Tile::Start,
            _ => unimplemented!()
        }
    }

    fn is_key_or_door(&self) -> bool {
        self.is_key() || self.is_door()
    }

    fn is_key(&self) -> bool {
        match self {
            Tile::Key(_) => {true},
            _ => false
        }
    }

    fn is_wall(&self) -> bool {
        match self {
            Tile::Wall => true,
            _ => false
        }
    }

    fn is_door(&self) -> bool {
        match self {
            Tile::Door(_) => {true},
            _ => false
        }
    }

    fn walkable(&self) -> bool {
        *self != Tile::Wall
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let c =match self {
            Tile::Wall => {'#'},
            Tile::Empty => {'.'},
            Tile::Key(c) => {*c},
            Tile::Door(c) => {*c},
            Tile::Start => {'@'},
        };
        f.write_str(&c.to_string());
        Ok(())
    }
}

type Cache = Arc<RwLock<HashMap<(Vector2, Vec<Tile>), Vec<(Tile, i32, Vector2)>>>>;

pub struct Map {
    data: Vec<Vec<Tile>>,
    target_cache: Cache
}

impl Map {
    fn new(data: Vec<Vec<Tile>>) -> Self {
        Map {
            data,
            target_cache: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    fn height(&self) -> usize {
        self.data.len()
    }

    fn width(&self) -> usize {
        self.data[0].len()
    }

    fn get(&self, pos: &Vector2) -> Option<Tile> {
        if (0..self.height()).contains(&(pos.y as usize)) &&
            (0..self.width()).contains(&(pos.x as usize)) {
            Some(self.data[pos.y as usize][pos.x as usize])
        } else {
            None
        }
    }

    fn set(&mut self, pos: &Vector2, value: Tile) {
        if (0..self.height()).contains(&(pos.y as usize)) &&
            (0..self.width()).contains(&(pos.x as usize)) {
            self.data[pos.y as usize][pos.x as usize] = value;
        }
    }

    fn all_tiles(&self) -> Vec<(Vector2, Tile)> {
        self.data.iter()
            .enumerate()
            .flat_map(|(y, v)|
                v.iter().enumerate().map(move |(x, &t)| (Vector2::new(x as i32, y as i32), t))
            ).collect()
    }

    fn analyse(&self) -> HashMap<Tile, Vector2> {
        let mut hash_map = HashMap::new();
        self.all_tiles().iter().for_each(|&(pos, tile)| {
            match tile {
                Tile::Wall => {},
                Tile::Empty => {},
                Tile::Key(_) => { hash_map.insert(tile, pos); },
                Tile::Door(_) => { hash_map.insert(tile, pos); },
                Tile::Start => { hash_map.insert(tile, pos); },
            }
        });
        hash_map
    }

    fn possible_targets(&self, start: &Vector2, mut keys: Vec<Tile>) -> Vec<(Tile, i32, Vector2)>{
        keys.sort_by_key(|t| {
            if let Tile::Key(c) = *t {
                return c
            } else {
                return '0'
            }
        });
        let cache_key = (*start, keys.clone());
        if let Some(r) = self.target_cache.read().unwrap().get(&cache_key) {
            return r.to_vec();
        }
        let mut to_visit = VecDeque::new();
        let mut visited = HashSet::new();
        let start = (self.get(start).unwrap(), 0, start.clone());
        to_visit.push_back(start);
        let mut ordered = vec![];
        while let Some((t, dist, v)) = to_visit.pop_front() {
            if visited.contains(&v) {
                continue;
            }
            visited.insert(v);
            if  t.is_key() && !keys.contains(&t) {
                ordered.push((t, dist, v.clone()));
                continue
            }

            if let Tile::Door(c) = t {
                let corresponding_key =  c.to_ascii_lowercase();
                if !keys.contains(&Tile::Key(corresponding_key)) {
                    continue
                }
            }

            for n in v.neigh() {
                if let Some(tile) = self.get(&n) {
                    if tile.walkable() {
                        to_visit.push_back((tile, dist + 1, n));
                    }
                }
            }

        }
        self.target_cache.write().unwrap().insert(cache_key, ordered.clone());
        ordered
    }

    fn run(&self, start: &Vector2) -> i32 {
        let mut path = self.batch_brute(*start);
        path.iter().rev().for_each(|t| {
            print!("{} ", *t);
        });
        println!();
        let mut min_score = self.compute_path(*start, path.clone()).unwrap();
        println!("Before optim: {}", min_score);
        for i in 0..path.len() {
            for j in 0..path.len() {
                if i <= j {
                    continue
                }
                let mut new_path = path.clone();
                new_path.swap(i,j);
                if let Some(d) = self.compute_path(*start, new_path.clone()) {
                    if d < min_score {
                        min_score = d;
                        path = new_path;
                    }
                }
            }
        }
        path.iter().rev().for_each(|t| {
            print!("{} ", *t);
        });
        println!();
        self.compute_path(*start, path).unwrap()
    }

    fn run_multi_brute(&self, robot_pos: Vec<Vector2>, keys: Vec<Tile>) -> i32 {
        robot_pos.iter().enumerate()
            .filter(|(i,p)| {
                self.possible_targets(p, keys.clone()).len() > 0
            })
            .map(|(i,p)| {
            let (d, new_p, path) = self.run_brute(p, keys.clone(), None);
            let mut new_all_pos = robot_pos.clone();
            let mut new_keys = keys.clone();
            for k in path {
                new_keys.push(k);
            }
            new_all_pos[i] = new_p;
            d + self.run_multi_brute(new_all_pos, new_keys)
        }).min().unwrap_or(0)
    }

    fn batch_brute(&self, mut pos: Vector2) -> Vec<Tile> {
        let mut keys = vec![];
        let mut brute_seq = vec![8,8];
        loop {
            let (_, new_pos, mut small_path) = self.run_brute(&pos, keys.clone(), brute_seq.pop());
            small_path.iter().rev().for_each(|t| {
                print!("{} ", *t);
                keys.push(*t);
            });
            pos = new_pos;
            println!();
            if small_path.len() == 0 {
                break
            }
        }
        keys.reverse();
        keys
    }

    fn compute_path(&self, start_pos: Vector2, path: Vec<Tile>) -> Option<i32> {
        let keys = vec![];
        self.run_replay(&start_pos, keys, path)
    }

    fn run_replay(&self, pos: &Vector2, mut keys: Vec<Tile>, mut path: Vec<Tile>) -> Option<i32> {
        let targets_keys = self.possible_targets(&pos, keys.clone());
        if let Some(p) = path.pop() {
            if let Some(&(t,d,v)) = targets_keys.iter().find(|(t, _, _)| *t == p) {
                keys.push(p);
                self.run_replay(&v, keys, path).map(|r| d + r)
            } else {
                None
            }
        } else {
            Some(0)
        }
    }

    fn run_brute(&self, pos: &Vector2, keys: Vec<Tile>, size: Option<usize>) -> (i32, Vector2, Vec<Tile>) {
        if let Some(s) = size {
            if s == 0 {
                return (0, *pos, vec![]);
            }
        }
        let targets_keys = self.possible_targets(&pos, keys.clone());

        targets_keys.iter().map(
            |(t, d, v)| {
                let mut keyset = keys.clone();
                keyset.push(*t);
                let (r, p, mut path) = self.run_brute(v, keyset, size.map(|s| s - 1));
                path.push(*t);
                (r+d, p, path)
            }
        ).min_by_key(|(d,_,_)|*d).unwrap_or((0, *pos, vec![]))
    }

}

pub enum Solution {}

impl Solution {

}

impl Solver for Solution {
    type Input = Map;
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, Error> {
        let m = input.lines()
            .map(|l| l.chars().map(Tile::from_char).collect_vec())
            .collect_vec();
        Ok(Map::new(m))
    }

    fn solve_part1(mut input: Self::Input) -> Result<Self::Output, Error> {
        let (start_pos, _) = input.all_tiles().into_iter().find(|&(v, t)| t == Tile::Start).unwrap();
        let res = input.run(&start_pos);
        Ok(res)
    }

    fn solve_part2(mut input: Self::Input) -> Result<Self::Output, Error> {
        let (start_pos, _) = input.all_tiles().into_iter().find(|&(v, t)| t == Tile::Start).unwrap();
        input.set(&start_pos, Tile::Wall);
        input.set(&start_pos.add(&Vector2::new(0,1)), Tile::Wall);
        input.set(&start_pos.add(&Vector2::new(0,-1)), Tile::Wall);
        input.set(&start_pos.add(&Vector2::new(1,0)), Tile::Wall);
        input.set(&start_pos.add(&Vector2::new(-1,0)), Tile::Wall);
        let start1 = start_pos.add(&Vector2::new(-1,-1));
        let start2 = start_pos.add(&Vector2::new(-1,1));
        let start3 = start_pos.add(&Vector2::new(1,-1));
        let start4 = start_pos.add(&Vector2::new(1,1));
        input.set(&start1, Tile::Start);
        input.set(&start2, Tile::Start);
        input.set(&start3, Tile::Start);
        input.set(&start4, Tile::Start);

        let mut rpos = vec![start1, start2, start3, start4];
        let mut total_d = input.run_multi_brute(rpos, vec![]);
        Ok(total_d)
    }
}

#[cfg(test)]
mod tests {

}