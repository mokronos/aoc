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
    let steps = 500;
    let total = solve2(&data, steps);

    println!("Task 21_2 Solution:\nTotal: {}", total);
}

fn solve(data: &str) -> u64
{
    let map = data.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let start: (usize, usize) = map.iter().enumerate()
        .find_map(|(y, l)| l.iter().enumerate().find_map(|(x, c)| if *c == 'S' { Some((x, y)) } else { None })).unwrap();
    let start = (start.0 as isize, start.1 as isize);

    let steps = 64;
    let plots = reach(&map, start, steps, false);
    
    plots as u64
}

fn solve2(data: &str, steps: usize) -> u64
{
    let map = data.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let start: (usize, usize) = map.iter().enumerate()
        .find_map(|(y, l)| l.iter().enumerate().find_map(|(x, c)| if *c == 'S' { Some((x, y)) } else { None })).unwrap();
    let start = (start.0 as isize, start.1 as isize);
    let toreach = 26501365;
    let mult = 2;
    let steps = toreach%map.len() + map.len() * mult;
    let mult_needed = toreach / map.len();
    let plots = reach(&map, start, steps, true);
    // f(steps) -> f(steps + length) -> f(steps + 2*length)
    // f(0) = 3752
    // f(1) = 33614
    // f(2) = 93252
    // extrapolate:
    // -> f(202300) = 609298746763952
    
    plots as u64
}

    
fn reach(map: &Vec<Vec<char>>, start: (isize, isize), steps: usize, overflow: bool) -> usize {

    const DIRS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    let mut counter = 0;
    q.push_back((start, (0, 0)));

    for step in 0..steps {
        let mut new_q = VecDeque::new();
        while q.len() > 0 {
            let (pos, grid) = q.pop_front().unwrap();
            for dir in DIRS {
                let mut next = (pos.0 + dir.0, pos.1 + dir.1);
                let mut next_grid = grid;
                if overflow {
                    (next, next_grid) = wrap_pos(map, next, grid);
                }
                if visited.contains(&(next, next_grid)) {
                    continue;
                }
                if map[next.1 as usize][next.0 as usize] == '#' {
                    continue;
                }
                visited.insert((next, next_grid));
                new_q.push_back((next, next_grid));
            }
        }
        if (steps - step) % 2 == 1 {
            counter += new_q.len();
        }
        q = new_q;
        // println!("Step: {}, Q: {:?}", step, q);
        // println!("Counter: {}", counter);
    }
    counter
}

fn wrap_pos(map: &Vec<Vec<char>>, pos: (isize, isize), grid: (isize, isize)) -> ((isize, isize), (isize, isize)) {
    if pos.0 < 0 {
        return (((map[0].len()-1) as isize, pos.1), (grid.0 - 1, grid.1));
    }
    if pos.0 >= map[0].len() as isize {
        return ((0, pos.1), (grid.0 + 1, grid.1));
    }
    if pos.1 < 0 {
        return ((pos.0, (map.len() - 1) as isize), (grid.0, grid.1 - 1));
    }
    if pos.1 >= map.len() as isize {
        return ((pos.0, 0), (grid.0, grid.1 + 1));
    }
    (pos, grid)
}

fn print_map(map: &Vec<Vec<char>>, visited: &HashSet<(isize, isize)>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if visited.contains(&(x as isize, y as isize)) {
                print!("O");
            } else {
                print!("{}", map[y][x]);
            }
        }
        println!();
    }
    println!();
}
