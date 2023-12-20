use std::collections::HashMap;

use itertools::Itertools;

use crate::solutions::Solver;

use crate::solutions::AocError;

pub enum Solution {}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct State {
    record: Vec<i32>,
    cur: usize,
    count: i32,
}

impl State {
    fn new(record: &[i32]) -> Self {
        State {
            record: record.to_owned(),
            cur: 0,
            count: 0,
        }
    }
    fn done(&self) -> bool {
        let record = self.record[self.cur];
        self.cur == self.record.len() - 1 && record == self.count
    }

    fn next(&mut self, elem: Elem) -> bool {
        let record = self.record[self.cur];
        match elem {
            Elem::Op => {
                if self.done() {
                    return true;
                }
                if self.count == 0 {
                    return true;
                }
                if record == self.count {
                    self.cur += 1;
                    self.count = 0;
                    return true;
                }
                return false;
            }
            Elem::Damaged => {
                let record = self.record[self.cur];
                if record == self.count {
                    return false;
                }
                self.count += 1;
                return true;
            }
            Elem::Unknown => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Input {
    elems: Vec<Elem>,
    s: State,
}

impl Solution {
    fn printElems(es: &[Elem]) {
        for e in es {
            match e {
                Elem::Damaged => print!("#"),
                Elem::Op => print!("."),
                Elem::Unknown => print!("?"),
            }
        }
        println!()
    }

    fn solve_1_cached(cache: &mut HashMap<Input, u64>, mut input: Input) -> u64 {
        if let Some(res) = cache.get(&input) {
            //print!("{:?} => ", input.s);
            //Solution::printElems(&input.elems);
            //println!("{}", res);
            return *res;
        } else {
            let initInput = input.clone();
            //println!()
            //;
            //print!("solve1:");
            //Solution::printElems(elems);
            let mut index = 0;
            while index < input.elems.len() {
                let cur = input.elems[index];
                //println!("{:?} {} {:?}", cur, index, input.s);
                match cur {
                    Elem::Unknown => {
                        let mut next_state = input.s.clone();
                        let mut res = 0;
                        if next_state.next(Elem::Op) {
                            //println!("b1: Op");
                            let ninput = Input {
                                elems: (&input.elems[index + 1..]).to_owned(),
                                s: next_state,
                            };
                            let r = Solution::solve_1_cached(cache, ninput);
                            res += r;
                        }
                        if input.s.next(Elem::Damaged) {
                            let ninput = Input {
                                elems: (&input.elems[index + 1..]).to_owned(),
                                s: input.s.clone(),
                            };
                            //println!("b2: Dmg");
                            let r = Solution::solve_1_cached(cache, ninput);
                            //println!("b2: {r}");
                            res += r;
                        }
                        if !cache.contains_key(&initInput) {
                            cache.insert(initInput, res);
                        }

                        return res;
                    }
                    f => {
                        if !input.s.next(f) {
                            if !cache.contains_key(&initInput) {
                                cache.insert(initInput, 0);
                            }
                            return 0;
                        }
                        index += 1;
                    }
                }
            }
            //println!("{:?}", state);
            let res = if input.s.done() { 1 } else { 0 };

            if !cache.contains_key(&initInput) {
                cache.insert(initInput, res);
            }
            return res;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Elem {
    Op,
    Damaged,
    Unknown,
}

pub struct Spring {
    elems: Vec<Elem>,
    record: Vec<i32>,
}

impl Solver for Solution {
    type Input = Vec<Spring>;
    type Output = u64;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        Ok(input
            .lines()
            .map(|l| {
                let (data, values) = l.split_once(' ').unwrap();
                let values = values.split(',').map(|c| c.parse().unwrap()).collect_vec();
                let elems = data
                    .chars()
                    .filter_map(|c| match c {
                        '.' => Some(Elem::Op),
                        '#' => Some(Elem::Damaged),
                        '?' => Some(Elem::Unknown),
                        _ => None,
                    })
                    .collect_vec();
                Spring {
                    elems,
                    record: values,
                }
            })
            .collect_vec())
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, AocError> {
        let mut sum = 0;

        for i in input {
            let min_length = i.record.iter().sum::<i32>() + i.record.len() as i32 - 1;
            //println!("{} {}", min_length, i.elems.len());
            let state = State::new(&i.record);

            let input = Input {
                s: state,
                elems: i.elems,
            };
            let mut cache = HashMap::new();

            let res = Solution::solve_1_cached(&mut cache, input);
            println!("{} {:?}", res, i.record);
            sum += res;
        }
        Ok(sum as u64)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, AocError> {
        let mut sum = 0;
        for i in input {
            let mut newrecord = vec![];
            let mut newElems = vec![];
            for _ in 0..5 {
                newrecord.append(&mut i.record.clone());
                if newElems.len() > 0 {
                    newElems.push(Elem::Unknown);
                }
                newElems.append(&mut i.elems.clone());
            }
            //println!("{} {}", min_length, i.elems.len());
            let state = State::new(&newrecord);
            let input = Input {
                s: state,
                elems: newElems,
            };
            let mut cache = HashMap::new();
            let res = Solution::solve_1_cached(&mut cache, input) as u64;

            sum += res;
        }
        Ok(sum)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::Solver;

    use super::{Solution, State};

    #[test]
    fn d12() {
        let i = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let i2 = "?#?????###? 2,4
";

        let i3 = "?###? 4";
        let i4 = "?# 2";
        let r = Solution::solve(i, false).unwrap();
        dbg!(r);
    }

    //#[test]
    fn testM() {
        let i = "#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1";
        let r = Solution::solve(i, true).unwrap();
        dbg!(r);
    }
}
