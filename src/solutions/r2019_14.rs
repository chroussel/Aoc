use failure::Error;
use crate::solutions::Solver;
use itertools::Itertools;
use nom::lib::std::collections::{HashMap, VecDeque};
use num::Integer;

#[derive(Debug, Clone)]
struct Schema {
    equations: Vec<Reaction>,
    reversed: HashMap<String, Reaction>,
    depth_map: HashMap<String, i64>
}

impl Schema {
    fn new(equations: Vec<Reaction>) -> Self {
        let mut map = HashMap::new();
        equations.iter().for_each(|r| {
            map.insert(r.product.1.clone(), r.clone());
        });
        let depths= Schema::depth_measure(&equations);
        Schema {
            equations,
            reversed: map,
            depth_map: depths
        }
    }

    fn depth_measure(equations: &Vec<Reaction>) ->HashMap<String, i64> {
        let mut depth_map = HashMap::new();
        depth_map.insert("ORE".to_string(), 1);
        let mut stack: VecDeque<Reaction> = equations.clone().into_iter().collect();
        while let Some(r) = stack.pop_front() {
            if r.regent.iter().all(|(_, element)| depth_map.contains_key(element)) {
                let d = r.regent.iter().map(|(_, element)| depth_map.get(element).unwrap()).max().unwrap() + 1;
                depth_map.insert(r.product.1.clone(), d);
            } else {
                stack.push_back(r);
            }
        }
        depth_map
    }

    fn solve(&mut self, fuel_count: i64) -> i64 {
        let mut r = HashMap::new();
        r.insert("FUEL".to_string(), fuel_count);
        while r.iter().any(|(key,_)| key != "ORE") {
            r = self.step_solve(&r);
        }
        *r.get("ORE").unwrap()
    }

    fn step_solve(&self, prev: &HashMap<String, i64>) -> HashMap<String, i64> {
        let mut new_map = HashMap::new();
        let max_depth = prev.iter().map(|(e, _)| *self.depth_map.get(e).unwrap()).max().unwrap();
        prev.into_iter().for_each(|(e, &count)| {
            let d = self.depth_map.get(e).unwrap();
            if *d == max_depth {
                let reaction = self.reversed.get(e).unwrap();
                let r_count = count.div_ceil(&reaction.product.0);
                reaction.regent.iter().for_each(|(r, ele)| {
                    let v = r * r_count;
                    new_map.entry(ele.clone())
                        .and_modify(|prev| *prev += v)
                        .or_insert(v);
                });
            } else {
                new_map.entry(e.clone())
                    .and_modify(|prev| *prev += count)
                    .or_insert(count);
            }
        });
        new_map
    }
}

pub enum Solution {}

#[derive(Debug, Clone)]
pub struct Reaction {
    regent: Vec<(i64, String)>,
    product: (i64, String)
}

fn regent_parser() {

}

fn parser(input: &str) -> Result<Reaction, Error> {
    let (regent_str, product_str): (&str,&str) = input.split("=>").collect_tuple().unwrap();
    let (p_count, p_elem): (&str,&str) = product_str.trim().split(" ").collect_tuple().unwrap();
    let p_count = p_count.trim().parse::<i64>().unwrap();
    let regent= regent_str.split(",")
        .map(|s| {
            let (count, elem): (&str,&str) = s.trim().split(" ").collect_tuple().unwrap();
            ((count.trim().parse::<i64>().unwrap()), elem.trim().to_string())
        }).collect_vec();

    Ok(Reaction {
        regent,
        product: (p_count, p_elem.trim().to_string())
    })
}

impl Solution {

}

impl Solver for Solution {
    type Input = Vec<Reaction>;
    type Output = i64;

    fn parse_input(input: &str) -> Result<Self::Input, Error> {
        input.lines()
            .map(|l| parser(l))
            .collect()
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, Error> {
        let mut schema = Schema::new(input);
        let res = schema.solve(1);
        Ok(res)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, Error> {
        let mut schema = Schema::new(input);
        let res = schema.solve(1);
        let target:i64 = 1000000000000;

        let mut fuel = target / res;
        let mut actual = schema.solve(fuel);
        let mut r = fuel..fuel*2;
        while r.end - r.start > 1 {
            let middle = (r.end + r.start) / 2;
            actual = schema.solve(middle);
            if actual < target {
                r = middle..r.end;
            } else {
                r = r.start..middle;
            }
        }
        Ok(r.start)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::r2019_14::Solution;
    use crate::solutions::Solver;

    #[test]
    fn e1() {
        let input = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
        let r = Solution::solve(input, true).unwrap();
        dbg!(r);
    }

    #[test]
    fn e2() {
        let input= "157 ORE => 5 A
165 ORE => 6 B
44 C, 5 D, 1 E, 29 A, 9 F, 48 G => 1 FUEL
12 G, 1 F, 8 H => 9 E
179 ORE => 7 H
177 ORE => 5 G
7 B, 7 H => 2 C
165 ORE => 2 F
3 B, 7 A, 5 G, 10 H => 8 D";
        let r = Solution::solve(input, true).unwrap();
        dbg!(r);
    }

    #[test]
    fn e3() {
        let input= "171 ORE => 8 A
7 B, 3 C, 9 D, 26 E, 1 F, 2 G, 1 H => 4 L
114 ORE => 4 I
14 J => 6 C
6 I, 18 K, 12 F, 7 L, 31 M, 37 N => 1 FUEL
6 F, 2 C, 8 B, 18 K, 1 E, 6 G, 1 H => 6 M
15 O, 2 P, 1 J => 6 B
13 F, 10 P, 3 H, 14 E, 2 G, 1 B => 1 N
5 C => 4 F
189 ORE => 9 K
1 G, 17 O, 3 D => 2 E
12 J, 27 A => 2 O
15 K, 12 I => 5 D
3 I, 2 J => 7 G
121 ORE => 7 J
7 D => 6 H
5 I, 4 J => 5 P";
        let r = Solution::solve(input, true).unwrap();
        dbg!(r);
    }
}