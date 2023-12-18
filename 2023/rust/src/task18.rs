use rust::read_data;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use rand::{thread_rng, Rng, seq::SliceRandom};

#[allow(unused)]
const TEST_DATA: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

pub fn run18_1()
{
    let data = read_data("data/data18.txt");
    // let data = TEST_DATA;
    let total = solve(&data);

    println!("Task 18_1 Solution:\nTotal: {}", total);
}

pub fn run18_2()
{
    let data = read_data("data/data18.txt");
    // let data = TEST_DATA;
    let total = solve2(&data);

    println!("Task 18_2 Solution:\nTotal: {}", total);
}

fn solve(data: &str) -> u64
{
    let instrs = data.lines().map(Instr::parse).collect::<Vec<_>>();
    let path = Path::new(instrs);
    // shoelace formula
    let inner_area = path.shoelace();
    let outer_trench = path.trench_len as u64;
    // pick's theorem
    inner_area + (outer_trench/2) + 1
}

fn solve2(data: &str) -> u64
{
    let instrs = data.lines().map(Instr::parse2).collect::<Vec<_>>();
    let path = Path::new(instrs);
    // shoelace formula
    let inner_area = path.shoelace();
    let outer_trench = path.trench_len as u64;
    // pick's theorem
    inner_area + (outer_trench/2) + 1
}

struct Path {
    trench_len: usize,
    corners: Vec<(isize, isize)>,
}

impl Path {
    fn new(instrs: Vec<Instr>) -> Self {
        let mut path = Path {
            trench_len: 0,
            corners: Vec::new(),
        };
        let mut x = 0;
        let mut y = 0;

        for instr in instrs {
            x = x + instr.dir.0 * instr.dist as isize;
            y = y + instr.dir.1 * instr.dist as isize;
            path.trench_len += instr.dist;
            path.corners.push((x, y));
        }
        path
    }

    fn shoelace(&self) -> u64 {
        let mut sum: isize = 0;
        for i in 0..self.corners.len() {
            let (x1, y1) = self.corners[i];
            let (x2, y2) = self.corners[(i + 1) % self.corners.len()];
            let det: isize = x1 * y2 - x2 * y1;
            sum += det
        }
        (sum.abs() / 2) as u64
    }
}

#[derive(Debug)]
struct Instr {
    dir: (isize, isize),
    dist: usize,
    color: String,
}

impl Instr {
    fn parse(line: &str) -> Self {
        let mut parts = line.split_whitespace();
        let dir = match parts.next().unwrap() {
            "R" => (1, 0),
            "L" => (-1, 0),
            "D" => (0, 1),
            "U" => (0, -1),
            _ => panic!("Invalid direction"),
        };
        let dist = parts.next().unwrap().parse::<usize>().unwrap();
        let color = parts.next().unwrap().to_string().replace("(", "").replace(")", "");
        Instr { dir, dist, color }
    }

    fn parse2(line: &str) -> Self {
        let mut parts = line.split_whitespace();
        let color = parts.last().unwrap().to_string().replace("(", "").replace(")", "").replace("#", "");
        let dir = color.chars().last().unwrap();
        let dir = match dir {
            '0' => (1, 0),
            '1' => (0, 1),
            '2' => (-1, 0),
            '3' => (0, -1),
            _ => panic!("Invalid direction"),
        };
        let dist = color.chars().take(5).collect::<String>();
        let dist = u64::from_str_radix(&dist, 16).unwrap() as usize;
        Instr { dir, dist, color }
    }
}
