use failure::Error;
use crate::solutions::Solver;
use itertools::Itertools;

pub struct Image {
    data: Vec<u32>
}

impl Image {
    fn new(data: Vec<u32>) -> Self {
        Image {data}
    }
}

pub enum Solution {}

impl Solution {

}

impl Solver for Solution {
    type Input = Vec<Image>;
    type Output = usize;

    fn parse_input(input: &str) -> Result<Self::Input, Error> {
        let w = 25;
        let h = 6;
        let datalength = w * h;
        let pixels:Vec<u32> = input.chars().map(|c| c.to_digit(10)).flatten().collect();
        let count = pixels.len() / datalength;
        let mut res = vec![];
        for c in 0..count {
            let data:Vec<u32> = pixels[c*datalength..(c+1)*datalength].to_vec();
            let i = Image::new(data);
            res.push(i);
        }
        Ok(res)
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, Error> {
        let l = input.iter().min_by_key(|a|a.data.iter().filter(|&&d|d == 0).count()).unwrap();
        let c1 = l.data.iter().filter(|&&a| a == 1).count();
        let c2 =  l.data.iter().filter(|&&a| a == 2).count();
        Ok(c1 * c2)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, Error> {
        let w = 25;
        let h = 6;
        let datalength = w * h;
        let mut res = vec![];

        for i in 0..datalength {
            for c in 0..input.len() {
                let p = input[c].data[i];
                if p ==1 || p == 0 {
                    res.push(p);
                    break
                }
            }
        }

        let rs = res.iter().map(|d| d.to_string()).join("");
        for hi in 0..h {
            let s = &rs[hi*w..(hi+1)*w];
            println!("{}", s);
        }
        Ok(0)
    }
}

#[cfg(test)]
mod tests {

}