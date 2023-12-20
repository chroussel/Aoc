use std::collections::HashSet;
use std::fmt::Display;

use itertools::Itertools;

use crate::solutions::Solver;

use crate::solutions::common::Map;
use crate::solutions::AocError;

pub enum Solution {}

impl Solution {
    fn count_energ(input: &Map<Elem>, pos: (i32, i32), dir: (i32, i32)) -> usize {
        let mut visited = HashSet::new();
        let mut visited2 = HashSet::new();
        let mut map = input.clone();
        let mut rays = vec![(pos, dir)];
        while let Some((mut pos, mut dir)) = rays.pop() {
            if !visited2.insert((pos, dir)) {
                continue;
            }

            loop {
                pos = (pos.0 + dir.0, pos.1 + dir.1);
                if pos.0 < 0
                    || pos.0 >= input.0.len() as i32
                    || pos.1 < 0
                    || pos.1 >= input.0[pos.0 as usize].len() as i32
                {
                    break;
                }
                map.0[pos.0 as usize][pos.1 as usize] = Elem::Energized;

                visited.insert(pos.clone());
                let cell = input.0[pos.0 as usize][pos.1 as usize];
                //println!("{:?} {:?} {:?}", pos, dir, cell);
                match cell {
                    Elem::Energized => {}
                    Elem::Empty => {}
                    Elem::MirrorDown => dir = (dir.1, dir.0),
                    Elem::MirrorUp => dir = (-dir.1, -dir.0),
                    Elem::SplitterH => {
                        if dir.0 != 0 {
                            rays.push((pos.clone(), (0, 1)));
                            rays.push((pos.clone(), (0, -1)));
                            break;
                        }
                    }
                    Elem::SplitterV => {
                        if dir.1 != 0 {
                            rays.push((pos.clone(), (1, 0)));
                            rays.push((pos.clone(), (-1, 0)));
                            break;
                        }
                    }
                }
            }
        }

        return visited.len();
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Elem {
    Empty,
    MirrorDown,
    MirrorUp,
    SplitterV,
    SplitterH,
    Energized,
}

impl Elem {
    fn from_char(c: char) -> Option<Elem> {
        match c {
            '.' => Some(Elem::Empty),
            '\\' => Some(Elem::MirrorDown),
            '/' => Some(Elem::MirrorUp),
            '|' => Some(Elem::SplitterV),
            '-' => Some(Elem::SplitterH),
            _ => None,
        }
    }
}

impl Display for Elem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Elem::Empty => ".",
            Elem::MirrorUp => "/",
            Elem::SplitterV => "|",
            Elem::SplitterH => "-",
            Elem::MirrorDown => "\\",
            Elem::Energized => "#",
        };

        write!(f, "{}", c)
    }
}

impl Solver for Solution {
    type Input = Map<Elem>;
    type Output = usize;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        let res = input
            .lines()
            .map(|l| l.chars().filter_map(|c| Elem::from_char(c)).collect_vec())
            .collect_vec();
        Ok(Map(res))
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, AocError> {
        let res = Solution::count_energ(&input, (0, -1), (0, 1));
        return Ok(res);
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, AocError> {
        let s = input.shape();

        let mut m = 0;

        for i in 0..s.0 {
            let v = Solution::count_energ(&input, (i as i32, -1), (0, 1));
            m = v.max(m);

            let v = Solution::count_energ(&input, (i as i32, s.1 as i32), (0, -1));
            m = v.max(m);
        }

        for j in 0..s.1 {
            let v = Solution::count_energ(&input, (-1, j as i32), (1, 0));
            m = v.max(m);

            let v = Solution::count_energ(&input, (s.0 as i32, j as i32), (-1, 0));
            m = v.max(m);
        }
        Ok(m)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::Solver;

    use super::Solution;

    #[test]
    fn d16() {
        let i = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

        dbg!(Solution::solve(i, true));
    }
}
