use failure::Error;
use crate::solutions::Solver;
use std::collections::HashMap;
use crate::solutions::common::Vector2;
use itertools::Itertools;
use nom::lib::std::collections::{VecDeque, HashSet};

pub enum Solution {}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Tile {
    Empty,
    Wall,
    Void,
    Label(char),
    Warp(String)
}

impl Tile {
    fn is_label(&self) -> bool {
        if let Tile::Label(_) = self {
            true
        } else {
            false
        }
    }
}

pub struct Map {
    named: HashMap<String, (Vector2, Vector2)>,
    not_connected: HashMap<String, Vector2>,
    map: Vec<Vec<Tile>>
}

impl Map {
    fn get(&self, n: &Vector2) -> Tile {
        self.map[n.y as usize][n.x as usize].clone()
    }

    fn is_inner_warp(&self, pos: &Vector2) -> bool {
        if let Tile::Warp(_) = self.get(pos) {
            if (8..(self.map.len() - 8)).contains(&(pos.y as usize))
                && (8..(self.map[pos.y as usize].len() - 8)).contains(&(pos.x as usize)) {
                return true
            }
        }
        return false;
    }

    fn run(&self, start: Vector2, end: Vector2, layer: bool, debug: bool) -> i32 {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut path:HashMap<(i32,Vector2), (i32, i32, Vector2)> = HashMap::new();
        queue.push_back((0, 0, start));
        while let Some((d, level, pos)) = queue.pop_front() {
            if level < 0 {
                continue
            }
            if debug {
                println!("{}, {}, ({},{})", d, level, pos.x, pos.y);
            }
            let level_pos = (level, pos);
            if visited.contains(&level_pos) {
                continue;
            }
            visited.insert(level_pos);
            if pos == end && level == 0{
                break
            }

            for mut n in pos.neigh() {
                let tile = self.get(&n);
                let mut next_d = d+1;
                let mut next_level = level;
                match tile {
                    Tile::Empty => {},
                    Tile::Warp(w) => {
                        next_d += 1;
                        if layer {
                            if self.is_inner_warp(&n) {
                                next_level += 1;
                            } else {
                                next_level -= 1;
                            }
                        }
                        let (p1, p2) = self.named.get(&w).unwrap();
                        if n == *p1 {
                            n = p2.clone();
                            if debug {
                                println!("warp: {} {} {} ({},{}) -> ({},{})", w, next_d, next_level, p1.x, p1.y, p2.x, p2.y)
                            }
                        } else {
                            n = p1.clone();
                            if debug {
                                println!("warp: {} {} {} ({},{}) -> ({},{})", w, next_d, next_level, p2.x, p2.y, p1.x, p1.y);
                            }
                        }
                    },
                    _ => {continue},
                }

                let neigh = (next_level, n);
                if visited.contains(&neigh) {
                    continue;
                }

                queue.push_back((next_d, next_level, n));
                path.entry(neigh)
                    .and_modify(|(d, l, p)| {
                        if next_d < *d {
                            *d = next_d;
                            *p = pos;
                            *l = level
                        }
                    })
                    .or_insert((next_d, level, pos));
            }
        }
        self.compute_path(end, path)
    }

    fn compute_path(&self, end: Vector2, path: HashMap<(i32, Vector2), (i32, i32, Vector2)>) -> i32 {
        let mut total_d = 0;
        let mut current_pos = (0, end);
        while let Some((d, level, next)) = path.get(&current_pos) {
            total_d += 1;
            if let Tile::Warp(_) = self.get(next) {
                total_d+=1;
            }
            current_pos = (*level, *next);
        }
        total_d
    }
}

impl Solution {

}

impl Solver for Solution {
    type Input = Map;
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, Error> {
        let mut data = input.lines().map(|l|
            l.chars().map(|c| {
                match c {
                    'A'..='Z' => {
                        Tile::Label(c)
                    },
                    '#' => {
                        Tile::Wall
                    }
                    '.' => {
                        Tile::Empty
                    }
                    _ => {
                        Tile::Void
                    }
                }
            }).collect_vec()
        ).collect_vec();
        let mut named_waiting = HashMap::new();
        let mut named = HashMap::new();
        for y in 0..(data.len()-1) {
            for x in 0..(data[y].len()-1) {
                if let Tile::Label(c1) = data[y][x] {
                    if let Tile::Label(c2) = data[y+1][x] {
                        let label = format!("{}{}", c1, c2);
                        let pos = if let Some(Tile::Empty) = data.get(y+2).map(|v|v[x].clone()) {
                            Vector2::new((x) as i32,(y+2) as i32)
                        } else {
                            Vector2::new((x) as i32,(y-1) as i32)
                        };
                        if named_waiting.contains_key(&label) {
                            let pair_pos = named_waiting.remove(&label).unwrap();
                            named.insert(label, (pos, pair_pos));
                        } else {
                            if !named.contains_key(&label) {
                                named_waiting.insert(label, pos);
                            }
                        }
                        continue
                    }
                    if let Tile::Label(c2) = data[y][x+1] {
                        let label = format!("{}{}", c1, c2);
                        let pos = if let Some(Tile::Empty) = data[y].get(x+2) {
                            Vector2::new((x+2) as i32,(y) as i32)
                        } else {
                            Vector2::new((x-1) as i32,(y) as i32)
                        };

                        if named_waiting.contains_key(&label) {
                            let pair_pos = named_waiting.remove(&label).unwrap();
                            named.insert(label, (pos, pair_pos));
                        } else {
                            if !named.contains_key(&label) {
                                named_waiting.insert(label, pos);
                            }
                        }
                        continue
                    }
                }
            }
        }

        named.iter().for_each(|(s, (p1, p2))| {
            dbg!(s);
            dbg!(p1, &data[p1.y as usize][p1.x as usize]);
            dbg!(p2, &data[p2.y as usize][p2.x as usize]);
            data[p1.y as usize][p1.x as usize] = Tile::Warp(s.clone());
            data[p2.y as usize][p2.x as usize] = Tile::Warp(s.clone());
        });
        Ok(Map {
            map:data,
            not_connected: named_waiting,
            named
        })
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, Error> {
        dbg!(&input.not_connected);
        let start = input.not_connected["AA"];
        let end = input.not_connected["ZZ"];
        let res = input.run(start, end, false, false);
        Ok(res)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, Error> {
        dbg!(&input.not_connected);
        let start = input.not_connected["AA"];
        let end = input.not_connected["ZZ"];
        dbg!(input.is_inner_warp(&start));
        dbg!(input.is_inner_warp(&end));
        let res = input.run(start, end, true, false);
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::r2019_20::Solution;
    use crate::solutions::Solver;

    #[test]
    fn e1() {
        let i = "         A     #
         A        #
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z    #
             Z    #
";
        let r = Solution::solve(i, true);
        dbg!(r);
    }
}