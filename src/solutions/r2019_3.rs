use failure::Error;
use crate::solutions::Solver;
use failure::_core::cmp::Ordering;
use std::ops::Range;
use std::collections::HashSet;
use std::iter::FromIterator;

pub enum Solution {}
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left
}

impl Direction {
    fn value(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0,1),
            Direction::Down => (0,-1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
        }
    }

    fn from_char(c: char) -> Direction {
        match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => unimplemented!()
        }
    }
}
#[derive(Debug)]
pub struct Move {
    dir: Direction,
    step: i32
}

impl Move {
    fn new(dir: Direction, step: i32) -> Move {
        Move {dir, step}
    }
    fn from_string(input: &str) -> Result<Move, Error>{
        Ok(Move::new(
            Direction::from_char(input.chars().nth(0).unwrap()),
            input[1..].parse()?,
        ))
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    x: i32,
    y: i32
}

impl Pos {
    fn new(x:i32, y: i32) -> Self{
        Pos {x,y}
    }

    fn add(&self, x: i32, y: i32) -> Self {
        Pos::new(self.x + x, self.y + y)
    }

    fn apply(&self, input: &Move) -> Vec<Pos> {
       let mut v = vec![];
        let mut ori = self.clone();
        let (x,y) = input.dir.value();
        for i in 0..input.step {
            ori = ori.add(x,y);
            v.push(ori)
        }
        v
    }

    fn len(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}
impl Solution {
    fn compute_path(path: Vec<Move>)-> Vec<Pos> {
        let start = Pos::new(0,0);
        let res = path.iter().fold(vec!(start), |mut p, n| {
            let l = p.last().unwrap();
            p.append(&mut l.apply(n));
            p
        });
        res
    }

    fn distance(dest: &Pos, path: &[Pos]) -> i32{
        path.iter().take_while(|&p| *p != *dest).count() as i32
    }
}

impl Solver for Solution {
    type Input = (Vec<Move>, Vec<Move>);
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, Error> {
        let mut wires:Vec<Vec<&str>> =
            input
                .lines()
                .map(|l| l.split(',').collect())
                .collect();

        let w0:Result<Vec<Move>, Error> = wires[0].iter().map(|l|Move::from_string(l)).collect();
        let w1:Result<Vec<Move>, Error> = wires[1].iter().map(|l|Move::from_string(l)).collect();
        assert_eq!(2, wires.len());
        Ok((w0?, w1?))
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, Error> {
        let (w0, w1) = input;
        let p0 = Solution::compute_path(w0);
        let p1 = Solution::compute_path(w1);
        let h0: HashSet<Pos>= HashSet::from_iter(p0.into_iter().skip(1));
        let h1: HashSet<Pos> = HashSet::from_iter(p1.into_iter().skip(1));
        let inter = h0.intersection(&h1);

        Ok(inter
            .map(|p|p.len()).min().unwrap() as i32)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, Error> {
        let (w0, w1) = input;
        let p0 = Solution::compute_path(w0);
        let p1 = Solution::compute_path(w1);
        let h0: HashSet<Pos>= HashSet::from_iter(p0.clone().into_iter().skip(1));
        let h1: HashSet<Pos> = HashSet::from_iter(p1.clone().into_iter().skip(1));
        let inter = h0.intersection(&h1);
        Ok(inter
            .map(
                |i| {
                    Solution::distance(i, &p0) + Solution::distance(i, &p1)
                }
            ).min().unwrap() as i32)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::r2019_3::Solution;
    use crate::solutions::Solver;

    #[test]
    fn e0() {
        let str_v = "R8,U5,L5,D3\nU7,R6,D4,L4";
        dbg!(Solution::solve(str_v, true));
    }

    #[test]
    fn e1() {
        let str_v = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        dbg!(Solution::solve(str_v, true));
    }
}