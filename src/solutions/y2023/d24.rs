use crate::solutions::Solver;
use itertools::Itertools;
use nalgebra::Vector3;
use z3::{ast::Ast, Config, Context, Optimize};

use crate::solutions::AocError;

pub enum Solution {}

impl Solution {}
struct Rect {
    top: i64,
    bottom: i64,
    left: i64,
    right: i64,
}

impl Solver for Solution {
    type Input = Vec<(Vector3<i64>, Vector3<i64>)>;
    type Output = i64;

    fn parse_input(input: &str) -> Result<Self::Input, AocError> {
        let r = input
            .lines()
            .map(|l: &str| {
                let (p, v) = l.split_once('@').unwrap();
                let p = Vector3::from_iterator(p.split(", ").map(|r| r.trim().parse().unwrap()));
                let v = Vector3::from_iterator(v.split(", ").map(|r| r.trim().parse().unwrap()));
                (p, v)
            })
            .collect_vec();
        Ok(r)
    }

    fn solve_part1(input: Self::Input) -> Result<Self::Output, AocError> {
        let min = 200000000000000f64;
        let max = 400000000000000f64;

        //let min = 7f64;
        //let max = 27f64;

        let mut curves = vec![];
        for (p, v) in input.iter() {
            let a = v.y as f64 / v.x as f64;
            let b = p.y as f64 - p.x as f64 * a;
            curves.push((a, b, p, v));
        }

        let mut count = 0;
        // (x -p) / v = t
        // cx + d = ax +B => (d - b) / (a - c)
        for (id1, (a, b, p1, v1)) in curves.iter().enumerate() {
            for (id2, (c, d, p2, v2)) in curves.iter().enumerate() {
                if id2 <= id1 {
                    continue;
                }
                let x = (d - b) / (a - c);
                let y = a * x + b;
                let t = (x - p1.x as f64) / v1.x as f64;
                let t2 = (x - p2.x as f64) / v2.x as f64;

                if t >= 0. && t2 >= 0. && x >= min && x <= max && y >= min && y <= max {
                    count += 1;
                }
            }
        }

        Ok(count)
    }

    fn solve_part2(input: Self::Input) -> Result<Self::Output, AocError> {
        // p + t *v = p1 + t*v1;
        // p+ t2 *v = p2 + t2*v2;
        // p + t3 *v = p3 + t3*v3;
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let opt = z3::Solver::new(&ctx);

        let px = z3::ast::Int::new_const(&ctx, "px");
        let py = z3::ast::Int::new_const(&ctx, "py");
        let pz = z3::ast::Int::new_const(&ctx, "pz");

        let vx = z3::ast::Int::new_const(&ctx, "vx");
        let vy = z3::ast::Int::new_const(&ctx, "vy");
        let vz = z3::ast::Int::new_const(&ctx, "vz");

        for (p, v) in input.iter() {
            let t = z3::ast::Int::fresh_const(&ctx, "t");
            let px1 = z3::ast::Int::from_i64(&ctx, p.x);
            let py1 = z3::ast::Int::from_i64(&ctx, p.y);
            let pz1 = z3::ast::Int::from_i64(&ctx, p.z);

            let vx1 = z3::ast::Int::from_i64(&ctx, v.x);
            let vy1 = z3::ast::Int::from_i64(&ctx, v.y);
            let vz1 = z3::ast::Int::from_i64(&ctx, v.z);

            opt.assert(&(&px + &t * &vx)._eq(&(&px1 + &t * &vx1)));
            opt.assert(&(&py + &t * &vy)._eq(&(&py1 + &t * &vy1)));
            opt.assert(&(&pz + &t * &vz)._eq(&(&pz1 + &t * &vz1)));
        }

        println!("Run");

        let a = opt.check();
        println!("{:?}", a);
        let r = opt.get_model().unwrap();
        let px = r.eval(&px, true).unwrap().as_i64().unwrap();
        let py = r.eval(&py, true).unwrap().as_i64().unwrap();
        let pz = r.eval(&pz, true).unwrap().as_i64().unwrap();
        let vx = r.eval(&vx, true).unwrap().as_i64().unwrap();
        let vy = r.eval(&vy, true).unwrap().as_i64().unwrap();
        let vz = r.eval(&vz, true).unwrap().as_i64().unwrap();

        println!("{} {} {} {} {} {}", px, py, pz, vx, vy, vz);
        let res = px + py + pz;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::Solver;

    use super::Solution;

    #[test]
    fn d24() {
        let i = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

        dbg!(Solution::solve(i, true));
    }
}
