use failure::Error;
use itertools::*;
use crate::solutions::Solver;
use std::collections::HashMap;

pub enum Solution {}

impl Solution {
    fn path_to_root(key: &String, inverse_map: &HashMap<String, String>) -> Vec<String> {
        let mut ckey = key;
        let mut path = vec!(ckey.clone());
        while let Some(res) = inverse_map.get(ckey) {
            path.push(res.clone());
            ckey = res;
        }
        path
    }

    fn find_leaves(root: String, map: &HashMap<String, Vec<String>>) -> Vec<String> {
        let mut leaves = vec![];
        let mut current_node = vec!(root);
        while let Some(key) = current_node.pop() {
            if let Some(childs) = map.get(&key) {
                for c in childs {
                    current_node.push(c.clone())
                }
            } else {
                leaves.push(key.clone())
            }
        }
        leaves
    }
}

impl Solver for Solution {
    type Input = (HashMap<String, Vec<String>>, HashMap<String, String>);
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, Error> {
        let mut map = HashMap::new();
        let mut inverse_map = HashMap::new();
        input.lines()
            .map(|l| {
                   let mut iter= l.split(')');
                (iter.next().unwrap().to_string(), iter.next().unwrap().to_string())
            })
            .for_each(|tuple| {
                let (key, value) = tuple;
                map.entry(key.clone())
                    .and_modify(|v:&mut Vec<String>| {v.push(value.clone());})
                    .or_insert(vec!(value.clone()));
                inverse_map.insert(value.clone(), key.clone());
            });
        Ok((map, inverse_map))
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, Error> {
        let (map, inverse_map) = input;
        let nodes: Vec<String> = map.iter().flat_map(|(key, values)| values.clone()).collect();
        let mut c = 0;
        for l in nodes {
            let path = Solution::path_to_root(&l, &inverse_map);
            let ic = path.len() - 1;
            c += ic;
        }
        Ok(c as i32)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, Error> {
        let (map, inverse_map) = input;

        let mut path_you = Solution::path_to_root(&String::from("YOU"), &inverse_map);
        let mut path_san = Solution::path_to_root(&String::from("SAN"), &inverse_map);

        while let (Some(s), Some(y)) = (path_san.pop(), path_you.pop()) {
            if s != y {
                break
            }
        }

        let r = path_you.len() + path_san.len();
        Ok(r as i32)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::r2019_6::*;
    use crate::solutions::Solver;

    #[test]
    fn e1() {
        let i = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
        let res = Solution::solve(i, true).unwrap();
        println!("{}", res);
    }

    #[test]
    fn e2() {
        let i = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
        let res = Solution::solve(i, false).unwrap();
        println!("{}", res);
    }
}