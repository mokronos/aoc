use rust::read_data;
use std::collections::HashMap;

#[allow(unused)]
const TEST_DATA: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

pub fn run9_1()
{
    let data = read_data("data/data9.txt");
    // let data = TEST_DATA;
    let mut total = 0;
    let mut data = Data::new(&data);
    (_,total) = solve(&mut data);

    println!("Task 9_1 Solution:\nTotal: {}", total);
}

pub fn run9_2()
{
    let data = read_data("data/data9.txt");
    // let data = TEST_DATA;
    let mut total = 0;
    let mut data = Data::new(&data);
    (total,_) = solve(&mut data);

    println!("Task 9_2 Solution:\nTotal: {}", total);
}

#[derive(Debug)]
struct Data
{
    lines: Vec<Line>,
}

impl Data
{
    fn new(data: &str) -> Self
    {
        let mut lines = Vec::new();
        for line in data.lines()
        {
            let line = Line::new(line);
            lines.push(line);
        }
        Self
        {
            lines,
        }
    }

}

#[derive(Debug)]
struct Line
{
    numbers: Vec<Vec<i32>>,
}

impl Line
{
    fn new(line: &str) -> Self
    {
        let mut numbers = Vec::new();
        let mut cur = Vec::new();
        for number in line.split_whitespace()
        {
            cur.push(number.parse::<i32>().unwrap());
        }
        numbers.push(cur);
        Self
        {
            numbers,
        }
    }

    fn check_zero(&self) -> bool
    {
        self.numbers[self.numbers.len()-1].iter().all(|&x| x == 0)
    }
}


fn solve(data: &mut Data) -> (i32,i32) {
    let mut total_back = 0;
    let mut total_fwd = 0;
    for line in &mut data.lines
    {
        let (back_res, fwd_res) = solve_line(line);
        total_back += back_res;
        total_fwd += fwd_res;
    }
    (total_back, total_fwd)
}

fn solve_line(line: &mut Line) -> (i32,i32) {
    while !line.check_zero()
    {
        let cur_line = &line.numbers[line.numbers.len()-1];
        let mut next: Vec<i32> = Vec::new();
        for i in 0..cur_line.len()-1
        {
            let cur = cur_line[i+1]-cur_line[i];
            next.push(cur);
        }
        line.numbers.push(next);
    }

    let mut fwd_res = 0;
    for l in line.numbers.iter().rev().skip(1)
    {
        fwd_res = l[l.len()-1] + fwd_res;
    }
    let mut back_res = 0;
    for l in line.numbers.iter().rev().skip(1)
    {
        // old_back_res = l[0] - back_res;
        // back_res = l[0] - old_back_res;
        back_res = l[0] - back_res;
    }
    
    (back_res, fwd_res)
}

