use disjoint::DisjointSet;
use disjoint::DisjointSetVec;
use itertools::Itertools;
use rand::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use crate::solutions::Solver;

use crate::solutions::AocError;

pub enum Solution {}

impl Solution {}

fn bfs(
    compos: &HashMap<String, HashSet<String>>,
    flow: &HashMap<(String, String), (i32, i32)>,
    source: String,
    sink: String,
) -> Option<Vec<String>> {
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(source.clone());

    let mut found = false;
    while let Some(n) = queue.pop_front() {
        if n == sink {
            found = true;
            break;
        }
        if let Some(neigh) = compos.get(&n) {
            for ne in neigh.iter() {
                if !visited.contains_key(ne) && ne != &source {
                    let (c, f) = flow.get(&(n.clone(), ne.clone())).unwrap();
                    if c > f {
                        visited.insert(ne, n.clone());
                        queue.push_back(ne.clone());
                    }
                }
            }
        }
    }
    if !found {
        return None;
    }

    let mut path = vec![];
    let mut prev = sink.clone();
    path.push(sink.clone());
    while let Some(n) = visited.get(&prev) {
        path.push(n.clone());
        prev = n.clone();
    }
    path.reverse();
    return Some(path);
}

impl Solver for Solution {
    type Input = Vec<(String, String)>;
    type Output = i32;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        let mut hm = vec![];
        for l in input.lines() {
            let (name, compo) = l.split_once(':').unwrap();
            for c in compo.trim().split(' ') {
                hm.push((name.to_owned(), c.to_owned()));
                //hm.push((c.to_owned(), name.to_owned()));
            }
        }
        Ok(hm)
    }

    fn solve_part1(edges: Self::Input) -> Result<Self::Output, AocError> {
        let mut vertex = HashSet::new();
        for e in edges.iter() {
            vertex.insert(e.0.clone());
            vertex.insert(e.1.clone());
        }

        let mut cut: HashSet<(String, String)>;
        let mut g1: usize;
        let mut g2: usize;
        let mut minsofar = usize::MAX;
        loop {
            (cut, g1, g2) = mincut(&edges, &vertex);
            if cut.len() == 3 {
                break;
            } else if minsofar > cut.len() {
                minsofar = cut.len();
                println!("{:?}", minsofar);
            }
        }

        println!("{:?} {} {}", cut, g1, g2);
        Ok((g1 * g2) as i32)
    }

    fn solve_part2(_input: Self::Input) -> Result<Self::Output, AocError> {
        unimplemented!()
    }
}

fn mincut(
    edges: &Vec<(String, String)>,
    vertex: &HashSet<String>,
) -> (HashSet<(String, String)>, usize, usize) {
    let mut allset = DisjointSet::with_len(vertex.len());
    let mut p = HashMap::new();
    for (pos, v) in vertex.iter().enumerate() {
        p.insert(v.clone(), pos);
    }

    let mut rand = rand::thread_rng();
    let mut l = vertex.len();
    while l > 2 {
        let (a, b) = edges.iter().choose(&mut rand).unwrap();
        let p1 = p.get(a).unwrap();
        let p2 = p.get(b).unwrap();
        if allset.join(*p1, *p2) {
            l -= 1;
        }
    }

    let mut cutset = HashSet::new();
    for (a, b) in edges {
        let p1 = p.get(a).unwrap();
        let p2 = p.get(b).unwrap();

        if !allset.is_joined(*p1, *p2) {
            cutset.insert((a.clone(), b.clone()));
        }
    }

    let sets = allset.sets();

    (cutset, sets[0].len(), sets[1].len())
}

#[cfg(test)]
mod tests {

    use crate::solutions::Solver;

    use super::Solution;
    #[test]
    fn d25() {
        let i = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

        dbg!(Solution::solve(i, true));
    }
}
