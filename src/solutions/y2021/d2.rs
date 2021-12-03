use crate::solutions::{Solver, AocError};
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::{digit1, space0};
use nom::combinator::map_res;
use nom::sequence::tuple;
use nom::{IResult};

pub enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Command {
    fn from_text(t: &str, value: i32) -> Self {
        match t.to_lowercase().as_str() {
            "forward" => Command::Forward(value),
            "down" => Command::Down(value),
            "up" => Command::Up(value),
            _ => panic!("Invalid command: {}", t),
        }
    }

    fn execute(&self, sub: &mut Submarine) {
        match self {
            Command::Forward(i) => {
                sub.position += i;
                sub.depth += i * sub.aim;
            },
            Command::Down(i) => sub.aim += i,
            Command::Up(i) => sub.aim -= i,
        }
    }
}

struct Submarine {
    pub aim: i32,
    pub depth: i32,
    pub position: i32,
}
pub enum Solution {}

impl Solution {
    fn pattern(input: &str) -> IResult<&str, (&str, &str, &str)>  {
        tuple((
            alt((
                tag_no_case("forward"),
                tag_no_case("down"),
                tag_no_case("up"),
            )),
            space0,
            digit1,
        ))(input)
    }

    fn parse_line(line: &str) -> Result<Command, AocError> {
        let mut parser = map_res(
            Solution::pattern,
            |(s, _, d)|
            d.parse::<i32>().map(|i| Command::from_text(s, i)),
        );
        let (_, res) = parser(line).map_err(|_| AocError::Unknown("nom".into()))?;

        Ok(res)
    }
}

impl Solver for Solution {
    type Input = Vec<Command>;
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        input.lines().map(Solution::parse_line).collect()
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, AocError> {
        let mut sub = Submarine {
            depth: 0,
            aim: 0,
            position: 0
        };
        for c in input {
            c.execute(&mut sub);
        }

        Ok(sub.depth * sub.position)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, AocError> {
        let mut sub = Submarine {
            depth: 0,
            aim: 0,
            position: 0
        };
        for c in input {
            c.execute(&mut sub);
        }

        Ok(sub.depth * sub.position)
    }
}

#[cfg(test)]
mod tests {}
