use failure::Error;
use crate::solutions::Solver;
use nom::IResult;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while;
use nom::sequence::{delimited, tuple, preceded};
use nom::combinator::map;
use num::{signum, Integer};
use crate::solutions::common::Vector3;


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Moon {
    pos: Vector3,
    speed: Vector3
}

impl Moon {
    fn kinetic_energy(&self) -> i32 {
        self.speed.x.abs() + self.speed.y.abs() + self.speed.z.abs()
    }

    fn potential_energy(&self) -> i32 {
        self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()
    }

    fn total_energy(&self) -> i32 {
        self.kinetic_energy() * self.potential_energy()
    }

    fn apply_velocity(&mut self) {
        self.pos = self.pos.add(self.speed)
    }

    fn apply_gravity(&mut self, b: &Moon) {
        let sx = signum(self.pos.x - b.pos.x);
        let sy = signum(self.pos.y - b.pos.y);
        let sz = signum(self.pos.z - b.pos.z);

        self.speed.x -= sx;
        self.speed.y -= sy;
        self.speed.z -= sz;
    }

    fn print(&self) {
        println!("x: {}, y: {}, z:{} | vx: {}, vy: {}, vz: {} | energy: {}", self.pos.x, self.pos.y, self.pos.z, self.speed.x, self.speed.y, self.speed.z, self.total_energy())
    }
}

fn parse_char_str(v: &str)-> i32 {
    v.parse().unwrap()
}

fn v3_parser(input: &str) -> IResult<&str, Vector3> {
    delimited(
        tag("<"),
        map(
            tuple(
                (
                    preceded(
                        tag("x="),
                        map(
                            take_while(|c: char| c.is_digit(10) || c == '-'),
                            |o:&str| parse_char_str(o))
                    ),
                    preceded(
                        tag(", y="),
                        map(
                            take_while(|c: char| c.is_digit(10) || c == '-'),
                            |o:&str| parse_char_str(o))
                    ),
                    preceded(
                        tag(", z="),
                        map(
                            take_while(|c: char| c.is_digit(10) || c == '-'),
                            |o:&str| parse_char_str(o))
                    )
                )
            ),
            |(x, y, z)| Vector3::new(x, y, z)),
        tag(">"))(input)
}

#[derive(Clone, Eq, PartialEq)]
pub struct Universe {
    moons: Vec<Moon>,
}

impl Universe {
    fn new(moons: Vec<Moon>) -> Self {
        Universe { moons}
    }


    fn eqx(&self, other: &Universe) -> bool{
        for i in 0..self.moons.len() {
            let s = &self.moons[i];
            let o = &other.moons[i];

            if s.pos.x != o.pos.x || s.speed.x != o.speed.x {
                return false;
            }
        }
        true
    }

    fn eqy(&self, other: &Universe) -> bool{
        for i in 0..self.moons.len() {
            let s = &self.moons[i];
            let o = &other.moons[i];

            if s.pos.y != o.pos.y || s.speed.y != o.speed.y {
                return false;
            }
        }
        true
    }

    fn eqz(&self, other: &Universe) -> bool{
        for i in 0..self.moons.len() {
            let s = &self.moons[i];
            let o = &other.moons[i];

            if s.pos.z != o.pos.z || s.speed.z != o.speed.z {
                return false;
            }
        }
        true
    }

    fn apply_gravitiy(&mut self) {
        let copy_moons = self.moons.to_vec();
        for i in 0..self.moons.len() {
            let a = self.moons.get_mut(i).unwrap();
            for j in 0..copy_moons.len() {
                if i != j {
                    let b = copy_moons.get(j).unwrap();
                    a.apply_gravity(b);
                }
            }
        }
    }

    fn step(&mut self) {
        self.apply_gravitiy();
        self.moons.iter_mut().for_each(|m|m.apply_velocity());
    }

    fn print(&self) {
        println!("-----");
        for m in &self.moons {
            m.print();
        }
        println!("-----");
    }
}

pub enum Solution {}

impl Solution {

}

impl Solver for Solution {
    type Input = Vec<Moon>;
    type Output = u64;

    fn parse_input(input: &str) -> Result<Self::Input, Error> {
        input.lines()
            .map(|s| {
                let (_, o) = v3_parser(s).unwrap();
                let m = Moon {
                    pos:o,
                    speed: Vector3::zero()
                };
                Ok(m)
            }).collect()
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, Error> {
        let mut u = Universe::new(input);
        for i in 0..1000 {
            if i % 100 == 0 {
                println!("step: {}", i);
                u.print()
            }
            u.step();
        }
        Ok(u.moons.iter().map(|m| m.total_energy() as u64).sum())
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, Error> {
        let mut start = Universe::new(input);
        let mut u = start.clone();
        let mut step_count = 0;
        let mut period_x = 0_u64;
        let mut period_y = 0;
        let mut period_z = 0;

        loop {
            step_count +=1;
            u.step();

            if u.eqx(&start) {
                period_x = step_count;
            }

            if u.eqy(&start) {
                period_y = step_count;
            }

            if u.eqz(&start) {
                period_z = step_count;
            }
            if period_x !=0 && period_y != 0 && period_z !=0 {
                break
            }
        }

        Ok(period_x.lcm(&period_y).lcm(&period_z))
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::r2019_12::Solution;
    use crate::solutions::Solver;

    #[test]
    fn e0() {
        let input = "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";
        let res = Solution::solve(input, true).unwrap();
        dbg!(res);
    }

    #[test]
    fn e1() {
        let input = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

        let res = Solution::solve(input, false).unwrap();
        dbg!(res);
    }

}