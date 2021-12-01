use failure::Error;
use crate::solutions::Solver;

pub enum Solution {}

impl Solution {

}

impl Solver for Solution {
    type Input = Vec<i32>;
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, Error> {
        input.lines()
            .map(|s| s.parse::<i32>().map_err(|e| Error::from(e)))
            .collect()
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, Error> {
       let mut first = input[0];

       let mut count = 0;
       for i in input {
           if i > first {
               count +=1;
           }
           first = i;
       }
       Ok(count)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, Error> {
        let mut count = 0;
        let mut s = None;
        for w in input.windows(3) {
            let cs: i32 = w.iter().sum();
            if let Some(z) = s {
                if cs > z {
                    count +=1;
                }
            }
            s = Some(cs);
        }
        Ok(count)
    }
}

#[cfg(test)]
mod tests {

}