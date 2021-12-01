use failure::Error;
use crate::solutions::Solver;
use crate::solutions::intcode::Program;

pub enum Solution {}

impl Solution {
    fn is_tracker(input: &Program, x: i64, y: i64) -> bool {
        let mut o = input.clone().run_with_input(vec![y,x]);
        if let Some(o) = o.pop() {
            if o==1 {
               return true;
            }
        }
        return false;
    }
    fn check_square(input: &Program, x: i64, y: i64) -> bool {
        Solution::is_tracker(input,x,y) &&
            Solution::is_tracker(input,x+99, y) &&
            Solution::is_tracker(input, x, y+99) &&
            Solution::is_tracker(input, x+99,y+99)
    }
}

impl Solver for Solution {
    type Input = Program;
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, Error> {
        Program::parse(input)
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, Error> {
        let mut count = 0;
        for y in 0..50 {
            for x in 0..50 {
                let mut o = input.clone().run_with_input(vec![y,x]);
                if let Some(o) = o.pop() {
                    if o == 1 {
                        count +=1
                    }

                }
            }
        }
        Ok(count)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, Error> {
        let mut min_x = 0;
        let mut y = 800;
        loop {
            let mut wide = 0;
            let mut x = min_x;
            println!("y: {}, x:{}", y, x);
            loop {
                x+=1;
                if Solution::is_tracker(&input, x, y) {
                    wide+=1;
                } else {
                    if wide != 0 {
                        break;
                    } else {
                        min_x+=1;
                    }
                }

                if Solution::check_square(&input,x,y) {
                    return Ok((x * 10000 + y) as i32);
                }
            }
            y+=1;
        }
    }
}

#[cfg(test)]
mod tests {

}