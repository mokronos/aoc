use rust::read_data;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use rand::{thread_rng, Rng, seq::SliceRandom};

#[allow(unused)]
const TEST_DATA: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";


pub fn run21_1()
{
    let data = read_data("data/data21.txt");
    // let data = TEST_DATA;
    let total = solve(&data);

    println!("Task 21_1 Solution:\nTotal: {}", total);
}

pub fn run21_2()
{
    let data = read_data("data/data21.txt");
    // let data = TEST_DATA;
    let total = solve2(&data);

    println!("Task 21_2 Solution:\nTotal: {}", total);
}

fn solve(data: &str) -> u64
{
    0
}

fn solve2(data: &str) -> u64
{
    0
}
