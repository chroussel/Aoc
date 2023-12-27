use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::ops::Deref;
use std::ops::DerefMut;
use std::time::Instant;

use itertools::Itertools;

use crate::solutions::Solver;

use crate::solutions::AocError;

pub enum Solution {}

impl Solution {}

#[derive(Debug, Clone)]
pub enum MType {
    FlipFlop(bool),
    Conjonction(HashMap<String, bool>),
    Broadcast,
}

#[derive(Debug, Clone)]
pub struct Module {
    name: String,
    t: RefCell<MType>,
    output: Vec<String>,
}

impl Solver for Solution {
    type Input = HashMap<String, Module>;
    type Output = usize;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        let mut hm = HashMap::new();
        let mut input_count = HashMap::new();
        for l in input.lines() {
            let (n, output) = l.split_once(" -> ").unwrap();

            let mut chars = n.chars().peekable();
            let t = match chars.peek().unwrap() {
                '%' => {
                    _ = chars.next();
                    MType::FlipFlop(false)
                }
                '&' => {
                    _ = chars.next();
                    MType::Conjonction(HashMap::new())
                }
                _ => MType::Broadcast,
            };
            let name: String = chars.collect();

            let output: Vec<String> = output.split(", ").map_into().collect_vec();
            for o in output.iter() {
                input_count
                    .entry(o.clone())
                    .and_modify(|a: &mut Vec<String>| {
                        a.push(name.clone());
                    })
                    .or_insert(vec![name.clone()]);
            }
            let m = Module {
                name: name.clone(),
                t: RefCell::new(t),
                output,
            };
            hm.insert(name.clone(), m);
        }
        for (name, inputs) in input_count {
            if let Some(m) = hm.get(&name) {
                if let MType::Conjonction(h) = m.t.borrow_mut().deref_mut() {
                    for i in inputs {
                        h.insert(i, false);
                    }
                }
            } else {
            }
        }
        Ok(hm)
    }

    fn solve_part1(mut input: Self::Input) -> Result<Self::Output, AocError> {
        let mut high = 0;
        let mut low = 0;

        for _ in 0..1000 {
            let mut h: Vec<(String, String, bool)> =
                vec![("button".into(), "broadcaster".into(), false)];
            low += 1;
            let mut next_c: Vec<(String, String, bool)> = vec![];
            while h.len() > 0 {
                for (source, dest, s) in h.into_iter() {
                    if let Some(b) = input.get_mut(&dest) {
                        let mut news = false;
                        let mut should_send = true;
                        match b.t.get_mut() {
                            MType::FlipFlop(state) => {
                                if !s {
                                    *state = !*state;
                                    news = *state;
                                } else {
                                    should_send = false;
                                }
                            }
                            MType::Conjonction(states) => {
                                *states.get_mut(&source).unwrap() = s;
                                news = !states.values().all(|v| *v);
                            }
                            MType::Broadcast => {}
                        }
                        if should_send {
                            for o in b.output.iter() {
                                next_c.push((dest.clone(), o.clone(), news));
                            }
                            if news {
                                high += b.output.len();
                            } else {
                                low += b.output.len();
                            }
                        }
                    }
                }
                h = next_c;
                next_c = vec![];
            }
        }
        println!("{} {}", high, low);
        Ok(high * low)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, AocError> {
        let mut thc: HashMap<String, Option<usize>> = HashMap::new();

        let (_, module) = input
            .iter()
            .find(|a| a.1.output.contains(&"rx".to_owned()))
            .unwrap();
        if let MType::Conjonction(states) = module.t.borrow().deref() {
            for k in states.keys() {
                thc.insert(k.clone(), None);
            }
        }
        println!("{:?}", module);
        let mut notdone = true;
        let mut button = 0;
        while notdone {
            button += 1;
            let mut h: Vec<(&str, &str, bool)> = vec![("button", "broadcaster", false)];
            let mut next_c: Vec<(&str, &str, bool)> = vec![];
            while h.len() > 0 {
                for (source, dest, s) in h.into_iter() {
                    if let Some(b) = input.get(dest) {
                        let mut news = false;
                        let mut should_send = true;
                        match b.t.borrow_mut().deref_mut() {
                            MType::FlipFlop(state) => {
                                if !s {
                                    *state = !*state;
                                    news = *state;
                                } else {
                                    should_send = false;
                                }
                            }
                            MType::Conjonction(states) => {
                                if b.name == module.name && s && button > 1 {
                                    if let Some(v) = thc.get_mut(source) {
                                        if v.is_none() {
                                            println!("{} => {}", source, button);
                                            *v = Some(button);
                                        }
                                    }
                                }
                                *states.get_mut(source).unwrap() = s;
                                news = !states.values().all(|v| *v);
                            }
                            MType::Broadcast => {}
                        }
                        if should_send {
                            for o in b.output.iter() {
                                next_c.push((dest, o, news));
                            }
                        }
                    } else {
                        if dest == "rx" && !s {
                            println!("found {}", button);
                            return Ok(button);
                        }
                    }
                }
                h = next_c;
                next_c = vec![];
            }
            notdone = !thc.values().all(|v| v.is_some());
        }
        println!("{:?}", thc);
        let mut a = 1;
        for v in thc.into_values().filter_map(|a| a) {
            a = num::integer::lcm(a, v);
        }
        Ok(a)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::Solver;

    use super::Solution;

    #[test]
    fn d20() {
        let i = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

        let i2 = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        dbg!(Solution::solve(i, false));
        dbg!(Solution::solve(i2, false));
    }
}
