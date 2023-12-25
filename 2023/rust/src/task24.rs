use rust::read_data;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use rand::{thread_rng, Rng, seq::SliceRandom};
use z3::{Config, Context, Solver};
use z3::ast::{Ast, Int};

#[allow(unused)]
const TEST_DATA: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

pub fn run24_1()
{
    let data = read_data("data/data24.txt");
    // let data = TEST_DATA;
    let total = solve(&data);

    println!("Task 24_1 Solution:\nTotal: {}", total);
}

pub fn run24_2()
{
    let data = read_data("data/data24.txt");
    // let data = TEST_DATA;
    let total = solve2(&data);

    println!("Task 24_2 Solution:\nTotal: {}", total);
}

fn solve(data: &str) -> u64
{
    let lines = parse_data(&data);
    let minbound: i64 = 200_000_000_000_000;
    let maxbound: i64 = 400_000_000_000_000;
    let testarea: ((f64, f64), (f64, f64)) = ((minbound as f64, minbound as f64), (maxbound as f64, maxbound as f64));
    let mut total = 0;
    for (idx, line) in lines.iter().enumerate() {
        for line2 in &lines[idx+1..] {
            if line == line2 {
                continue;
            }
            let inter = intersection(line, line2);
            if inter.is_none() {
                continue;
            }
            let (x, y) = inter.unwrap();
            let outside = check_outside(&x, &y, &testarea);
            if outside {
                continue;
            }
            total += 1;
        }
    }
    total
}

fn solve2(data: &str) -> u64
{
    // x1 + vx1 * t1 = x_r + vx_r * t1
    // x2 + vx2 * t2 = x_r + vx_r * t2
    // x3 + vx3 * t3 = x_r + vx_r * t3
    // y1 + vy1 * t1 = y_r + vy_r * t1
    // y2 + vy2 * t2 = y_r + vy_r * t2
    // y3 + vy3 * t3 = y_r + vy_r * t3
    // z1 + vz1 * t1 = z_r + vz_r * t1
    // z2 + vz2 * t2 = z_r + vz_r * t2
    // z3 + vz3 * t3 = z_r + vz_r * t3
    // unknowns: t1, t2, t3, x_r, y_r, z_r, vx_r, vy_r, vz_r
    // 9 equations, 9 unknowns

    // z3 code, credit to https://github.com/arthomnix/aoc23/blob/master/src/days/day24.rs
    let hailstones = parse_data_i64(&data);
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let px = Int::new_const(&ctx, "px");
    let py = Int::new_const(&ctx, "py");
    let pz = Int::new_const(&ctx, "pz");
    let vx = Int::new_const(&ctx, "vx");
    let vy = Int::new_const(&ctx, "vy");
    let vz = Int::new_const(&ctx, "vz");

    for hailstone in hailstones {
        let pxn = Int::from_i64(&ctx, hailstone.px);
        let pyn = Int::from_i64(&ctx, hailstone.py);
        let pzn = Int::from_i64(&ctx, hailstone.pz);
        let vxn = Int::from_i64(&ctx, hailstone.vx);
        let vyn = Int::from_i64(&ctx, hailstone.vy);
        let vzn = Int::from_i64(&ctx, hailstone.vz);
        let tn = Int::fresh_const(&ctx, "t");

        solver.assert(&(&pxn + &vxn * &tn)._eq(&(&px + &vx * &tn)));
        solver.assert(&(&pyn + &vyn * &tn)._eq(&(&py + &vy * &tn)));
        solver.assert(&(&pzn + &vzn * &tn)._eq(&(&pz + &vz * &tn)));
    }

    solver.check();
    let model = solver.get_model().unwrap();
    let x = model.get_const_interp(&px).unwrap().as_i64().unwrap();
    let y = model.get_const_interp(&py).unwrap().as_i64().unwrap();
    let z = model.get_const_interp(&pz).unwrap().as_i64().unwrap();

    (x + y + z) as u64
}


#[derive(Debug, Clone, Copy, PartialEq)]
struct Line {
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct LineI64 {
    px: i64,
    py: i64,
    pz: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

fn  parse_data_i64(data: &str) -> Vec<LineI64>
{

    let mut lines = Vec::new();
    for (idx, line) in data.lines().enumerate() {
        let mut line = line.split("@");
        let mut pos = line.next().unwrap().split(",");
        let mut vel = line.next().unwrap().split(",");
        let px = pos.next().unwrap().trim().parse::<i64>().unwrap();
        let py = pos.next().unwrap().trim().parse::<i64>().unwrap();
        let pz = pos.next().unwrap().trim().parse::<i64>().unwrap();
        let vx = vel.next().unwrap().trim().parse::<i64>().unwrap();
        let vy = vel.next().unwrap().trim().parse::<i64>().unwrap();
        let vz = vel.next().unwrap().trim().parse::<i64>().unwrap();

        lines.push(LineI64 { px, py, pz, vx, vy, vz });
    }
    lines
}

fn  parse_data(data: &str) -> Vec<Line>
{

    let mut lines = Vec::new();
    for (idx, line) in data.lines().enumerate() {
        let mut line = line.split("@");
        let mut pos = line.next().unwrap().split(",");
        let mut vel = line.next().unwrap().split(",");
        let px = pos.next().unwrap().trim().parse::<f64>().unwrap();
        let py = pos.next().unwrap().trim().parse::<f64>().unwrap();
        let pz = pos.next().unwrap().trim().parse::<f64>().unwrap();
        let vx = vel.next().unwrap().trim().parse::<f64>().unwrap();
        let vy = vel.next().unwrap().trim().parse::<f64>().unwrap();
        let vz = vel.next().unwrap().trim().parse::<f64>().unwrap();

        lines.push(Line { px, py, pz, vx, vy, vz });
    }
    lines
}

fn intersection(a: &Line, b: &Line) -> Option<(f64, f64)>
{
    let ma = a.vy / a.vx;
    let mb = b.vy / b.vx;
    let ta = a.py - ma * a.px;
    let tb = b.py - mb * b.px;

    let x = (tb - ta) / (ma - mb);
    let y = ma * x + ta;
    if x.is_infinite() || y.is_infinite() {
        return None;
    }
    // check if the intersection is in the past
    for p in [a, b].iter() {
        if p.vx > 0.0 && x < p.px { return None; }
        if p.vx < 0.0 && x > p.px { return None; }
        if p.vy > 0.0 && y < p.py { return None; }
        if p.vy < 0.0 && y > p.py { return None; }
    }
    Some((x, y))
}

fn check_outside(x: &f64, y: &f64, area: &((f64, f64), (f64, f64))) -> bool
{
    if x < &area.0.0 || x > &area.1.0 { return true; }
    if y < &area.0.1 || y > &area.1.1 { return true; }
    false
}
