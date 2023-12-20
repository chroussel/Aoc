use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::multi::fold_many0;
use nom::multi::fold_many1;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::sequence::*;
use nom::IResult;

use crate::solutions::Solver;

use crate::solutions::AocError;

pub enum Solution {}

impl Solution {}

#[derive(Debug, Clone)]
pub enum Rule {
    Rejected,
    Accepted,
    Goto(String),
    Cond(Cond),
}

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    LESS,
    MORE,
}

#[derive(Debug, Clone)]

pub struct Cond(String, Dir, u64);

impl Cond {
    fn to_range(&self, res: bool) -> Range {
        if res {
            match self.1 {
                Dir::MORE => Range(self.0.to_owned(), self.2 + 1, 4001),
                Dir::LESS => Range(self.0.to_owned(), 1, self.2),
            }
        } else {
            match self.1 {
                Dir::LESS => Range(self.0.to_owned(), self.2, 4001),
                Dir::MORE => Range(self.0.to_owned(), 1, self.2 + 1),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Range(String, u64, u64);

impl Range {
    fn and(&self, other: &Range) -> Range {
        let m = self.1.max(other.1);
        let n = self.2.min(other.2);

        Range(self.0.clone(), m, n)
    }

    fn len(&self) -> u64 {
        self.2 - self.1
    }
}

#[derive(Debug, Clone)]
pub struct Workflow {
    name: String,
    cond: Vec<Rule>,
}

fn rule2(input: &str) -> IResult<&str, Rule> {
    let mut parser = alt((
        map(char('R'), |_| Rule::Rejected),
        map(char('A'), |_| Rule::Accepted),
        map(alpha1, |s: &str| Rule::Goto(s.to_owned())),
    ));

    parser(input)
}

fn rule(input: &str) -> IResult<&str, Vec<Rule>> {
    let cond1 = map(tuple((alpha1, take(1_usize), digit1)), |(a, sign, b)| {
        let d = match sign {
            ">" => Dir::MORE,
            "<" => Dir::LESS,
            _ => unimplemented!(),
        };
        let cond = Rule::Cond(Cond(a.to_owned(), d, b.parse().unwrap()));
        cond
    });

    let cond2 = alt((
        map(
            tuple((cond1, char(':'), rule2, char(','))),
            |(r1, _, r2, _)| (Some(r1), r2),
        ),
        map(rule2, |r| (None, r)),
    ));
    let mut parser = fold_many1(cond2, Vec::<Rule>::new, |mut acc, (optcond, ru)| {
        if let Some(cond) = optcond {
            acc.push(cond);
        }
        acc.push(ru);
        acc
    });

    let (rem, res) = parser(input)?;
    Ok((rem, res))
}

fn workflow(input: &str) -> Workflow {
    let delim_rule = delimited(char('{'), rule, char('}'));
    let mut parser = pair(alpha1, delim_rule);

    let (_, (name, res)) = parser(input).unwrap();
    Workflow {
        name: name.to_owned(),
        cond: res,
    }
}

fn recurse_count(
    w: &str,
    ws: &HashMap<String, Workflow>,
    validPaths: &mut Vec<Vec<Range>>,
    mut currentPath: Vec<Range>,
) {
    println!("{}", w);
    let w = ws.get(w).unwrap();
    for index in (0..w.cond.len()).step_by(2) {
        let c1 = &w.cond[index];
        let th = &w.cond[index + 1];
        let el = &w.cond[index + 2];

        if let Rule::Cond(cond) = c1 {
            let thrange = cond.to_range(true);
            let mut thpath = currentPath.clone();
            thpath.push(thrange);
            let elrange = cond.to_range(false);
            currentPath.push(elrange);
            match th {
                Rule::Rejected => {}
                Rule::Accepted => validPaths.push(thpath),
                Rule::Goto(w) => recurse_count(w, ws, validPaths, thpath),
                Rule::Cond(_) => {
                    unimplemented!()
                }
            }

            match el {
                Rule::Rejected => {
                    break;
                }
                Rule::Accepted => {
                    validPaths.push(currentPath.clone());
                    break;
                }
                Rule::Goto(w) => {
                    recurse_count(w, ws, validPaths, currentPath.clone());
                    break;
                }
                Rule::Cond(_) => {}
            }
        } else {
            unimplemented!();
        }
    }
}

#[derive(Debug, Clone)]
pub struct Part(HashMap<String, u64>);

fn partvalue(input: &str) -> IResult<&str, (&str, u64)> {
    let mut value = separated_pair(alpha1, tag("="), digit1);
    let (rem, (k, v)) = value(input)?;
    Ok((rem, (k, v.parse().unwrap())))
}
fn part(input: &str) -> Part {
    let values = separated_list1(char(','), partvalue);
    let mut parser = delimited(char('{'), values, char('}'));

    let (_, vs) = parser(input).unwrap();
    let mut hm = HashMap::new();

    for (k, v) in vs {
        hm.insert(k.to_owned(), v);
    }
    Part(hm)
}

impl Solver for Solution {
    type Input = (HashMap<String, Workflow>, Vec<Part>);
    type Output = u64;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        let mut lines = input.lines();

        let mut worflow = HashMap::new();
        let mut parts = vec![];
        while let Some(l) = lines.next() {
            if l == "" {
                break;
            }
            let w = workflow(l);
            worflow.insert(w.name.clone(), w);
        }

        while let Some(l) = lines.next() {
            let p = part(l);
            parts.push(p);
        }

        Ok((worflow, parts))
    }

    fn solve_part1((ws, ps): Self::Input) -> Result<Self::Output, AocError> {
        let mut sp = 0;
        for p in ps {
            let mut w = ws.get("in").unwrap();
            let mut index = 0;
            loop {
                let c = &w.cond[index];
                match c {
                    Rule::Rejected => {
                        break;
                    }
                    Rule::Accepted => {
                        let x = p.0["x"];
                        let m = p.0["m"];
                        let a = p.0["a"];
                        let s = p.0["s"];
                        sp += x + m + a + s;
                        break;
                    }
                    Rule::Goto(v) => {
                        w = ws.get(v).unwrap();
                        index = 0;
                    }

                    Rule::Cond(c2) => {
                        let var = *p.0.get(&c2.0).unwrap();
                        let res = match c2.1 {
                            Dir::LESS => var < c2.2,
                            Dir::MORE => var > c2.2,
                        };
                        if res {
                            index += 1;
                        } else {
                            index += 2;
                        }
                    }
                }
            }
        }

        Ok(sp as u64)
    }

    fn solve_part2((ws, _): Self::Input) -> Result<Self::Output, AocError> {
        let mut validPaths = vec![];
        recurse_count("in", &ws, &mut validPaths, vec![]);

        let mut sum = 0;
        for vp in validPaths {
            println!("{:?}", vp);
            let mut x = Range("x".into(), 1, 4001);
            let mut m = Range("m".into(), 1, 4001);
            let mut a = Range("a".to_owned(), 1, 4001);
            let mut s = Range("s".to_owned(), 1, 4001);

            for v in vp.iter() {
                match v.0.as_str() {
                    "x" => x = x.and(v),
                    "m" => m = m.and(v),
                    "a" => a = a.and(v),
                    "s" => s = s.and(v),
                    _ => {}
                }
            }
            println!("x: {:?}, m: {:?}, a: {:?}, s: {:?}", x, m, a, s);
            /*println!(
                "x: {:?}, m: {:?}, a: {:?}, s: {:?}",
                x.len(),
                m.len(),
                a.len(),
                s.len()
            );*/

            let res = x.len() * a.len() * m.len() * s.len();
            println!("{}", res);
            sum += res;
        }
        Ok(sum)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::Solver;

    use super::Solution;
    #[test]
    fn d19() {
        let i = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

        dbg!(Solution::solve(i, false));
    }
}
