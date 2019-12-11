use failure::Error;
use crate::solutions::Solver;
use itertools::Itertools;
use std::collections::HashSet;
use num::Integer;
use num::traits::FloatConst;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Position {
    Empty,
    Asteroid,
    Deleted(i32)
}

pub enum Solution {}

pub struct Map {
    data: Vec<Vec<Position>>,
}

impl Map {
    fn width(&self) -> i32 {
        self.data[0].len() as i32
    }

    fn height(&self) -> i32 {
        self.data.len() as i32
    }

    fn get(&self, point: Vector2) -> Position {
        self.data[point.y as usize][point.x as usize]
    }

    fn set(&mut self, point: Vector2, value: Position) {
        self.data[point.y as usize][point.x as usize] = value;
    }

    fn contains(&self, point: Vector2) -> bool {
        self.height() > point.y && self.width() > point.x && point.x >=0 && point.y >= 0
    }

    fn print(&self, point: Option<Vector2>) {
        println!("------------");
        for i in 0..self.height() {
            for j in 0..self.width() {
                let current = Vector2::new(j, i);
                if point.is_some() && current == point.unwrap(){
                    print!(" X ")
                } else {
                    match self.get(current) {
                        Position::Empty => print!(" . "),
                        Position::Asteroid => print!(" # "),
                        Position::Deleted(v) => print!("{:2} ",v)
                    }
                }
            }
            println!()
        }
        println!("------------");
    }

    fn ast_count(&self) -> usize {
        self.data.iter().map(|p| p.iter().filter(|&&p| p == Position::Asteroid).count()).sum()
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Vector2 {
    x: i32,
    y: i32
}

impl Vector2 {
    fn new(x: i32, y: i32) -> Self {
        Vector2 {x,y}
    }
    fn slope(&self, other: Vector2) -> Vector2 {
        let x = - self.x + other.x ;
        let y = - self.y + other.y;
        let g = x.gcd(&y);
        Vector2::new(x/g ,y/g)
    }

    fn angle(&self) -> f64 {
        let v = (self.y as f64).atan2(self.x as f64) ;
        let v2 = if v < 0.0 {
            v + 2.0 * f64::PI()
        } else {
            v
        };
        let v3 = v2 - f64::FRAC_PI_2() * 3.0;
        if v3 < 0.0 {
            v3 + 2.0 * f64::PI()
        } else {
            v3
        }
    }

    fn add(&self, other: Vector2) -> Vector2 {
        Vector2::new(self.x + other.x, self.y + other.y)
    }

    fn scale(&self, scalar: i32) -> Vector2 {
        Vector2::new(self.x * scalar, self.y * scalar)
    }
}

impl Solution {
    fn slope_list(map: &Map, center: Vector2) -> HashSet<Vector2> {
        let mut hashset = HashSet::new();
        for k in 0..map.height() {
            for h in 0..map.width() {
                let p = Vector2::new(h as i32, k as i32);
                if p == center {
                    continue;
                }
                if let Position::Asteroid = map.get(p) {
                    hashset.insert(center.slope(p));
                }
            }
        }
        hashset
    }

    fn find_point(map: &Map, slope: Vector2, pos: Vector2) -> Option<Vector2>{
        let mut current = pos.add(slope);
        let mut inc = 1;
        while map.contains(current) {
            if let Position::Asteroid = map.get(current) {
                return Some(current)
            }
            inc +=1;
            current = pos.add(slope.scale(inc));
        }
        None
    }

    fn sort_asteroid(map: &mut Map, center: Vector2) -> Vec<Vector2> {
        let mut slopes = Solution::slope_list(map, center).into_iter().collect_vec();
        slopes.sort_by(|&v, &v2| {
            v.angle().partial_cmp(&v2.angle()).unwrap()
        });

        let angles = slopes.iter().map(|&v| (v, v.angle())).collect_vec();
        //dbg!(&angles);
        let mut ast_count = map.ast_count();
        let mut res = vec![];
        //dbg!(ast_count);

        map.print(Some(center));
        let mut r = 1;
        for &s in slopes.iter().cycle() {
            if let Some(p) = Solution::find_point(map, s, center) {
                map.set(p, Position::Deleted(r));
                r+=1;
                res.push(p);
                ast_count -=1;
                map.print(Some(center));
            }
            if ast_count <= 1 {
                break;
            }
        }
        res
    }
}

impl Solver for Solution {
    type Input = Map;
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, Error> {
        let data = input
            .lines()
            .map(|l| {
                l.trim().chars().map(|c| {
                    match c {
                        '#' => Position::Asteroid,
                        '.' => Position::Empty,
                        _ => unimplemented!(),
                    }
                }).collect_vec()
            }).collect_vec();
        Ok(Map {
            data
        })
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, Error> {
        let mut m = 0;
        for i in 0..input.height() {
            for j in 0..input.width() {
                let current = Vector2::new(j,i);
                if let Position::Asteroid = input.get(current) {
                    m = m.max(Solution::slope_list(&input, current).len());
                }
            }
        }
        Ok(m as i32)
    }

    fn solve_part2(mut input: Self::Input) -> Result<Self::Output, Error> {
       let mut center = Vector2::new(0,0);
        let mut m = 0;
        for i in 0..input.height() {
            for j in 0..input.width() {
                let p = Vector2::new(j as i32,i as i32);
                if let Position::Asteroid = input.get(p) {
                    let v = Solution::slope_list(&input, p).len();
                    if v > m {
                        m = v;
                        center = p;
                    }
                }
            }
        }
        let sorted_ast = Solution::sort_asteroid(&mut input, center);
        let r = sorted_ast[199];
        Ok(r.x * 100 + r.y)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::r2019_10::{Solution, Vector2};
    use crate::solutions::Solver;

    #[test]
    fn e1() {
        let input = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
        let res = Solution::solve(input, true).unwrap();
        dbg!(res);
    }

    #[test]
    fn e2() {
        let input = ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##";
        let mut parsed = Solution::parse_input(input).unwrap();
        let c = Vector2::new(8, 3);
        let sorted = Solution::sort_asteroid(&mut parsed, c);
        parsed.print(Some(c));
        dbg!(sorted);
    }
}