use rust::read_data;
use std::collections::HashSet;

#[allow(unused)]
const TEST_DATA: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

pub fn run16_1()
{
    let data = read_data("data/data16.txt");
    // let data = TEST_DATA;
    let total = solve(&data);

    println!("Task 16_1 Solution:\nTotal: {}", total);
}

pub fn run16_2()
{
    let data = read_data("data/data16.txt");
    // let data = TEST_DATA;
    let total = solve2(&data);

    println!("Task 16_2 Solution:\nTotal: {}", total);
}

fn solve(data: &str) -> usize
{
    let map = data.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let start = Light { x: -1, y: 0, dir: (1, 0)};
    get_energized(&map, start)
}

fn solve2(data: &str) -> usize
{
    let map = data.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let starts = get_starts(&map);
    let mut best = 0;
    for start in starts {
        let cur = get_energized(&map, start);
        if cur > best {
            best = cur;
        }
    }
    best
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Light {
    x: isize,
    y: isize,
    dir: (isize, isize),
}

fn get_starts(map: &Vec<Vec<char>>) -> Vec<Light> {
    let mut starts = Vec::new();
    for i in 0..map.len() {
        let top_start = Light { x: -1, y: i as isize, dir: (1, 0)};
        let bottom_start = Light { x: map[0].len() as isize, y: i as isize, dir: (-1, 0)};
        starts.push(top_start);
        starts.push(bottom_start);
    }
    for i in 0..map[0].len() {
        let left_start = Light { x: i as isize, y: -1, dir: (0, 1)};
        let right_start = Light { x: i as isize, y: map.len() as isize, dir: (0, -1)};
        starts.push(left_start);
        starts.push(right_start);
    }
    starts.sort_by_key(|light| (light.x, light.y));
    starts
}

fn get_energized(map: &Vec<Vec<char>>, start: Light) -> usize {
    let mut queue = Vec::new();
    let mut visited = HashSet::new();
    let mut energized = HashSet::new();
    queue.push(start);

    while queue.len() > 0 {
        let light = queue.pop().unwrap();
        if visited.contains(&light) {
            continue;
        }
        visited.insert(light);
        energized.insert((light.x, light.y));
        let new_lights = step(&map, &light);
        queue.extend(new_lights);
    }
    energized.len() - 1
}
    

fn step(map: &Vec<Vec<char>>, light: &Light) -> Vec<Light> {

    let x = light.x + light.dir.0;
    let y = light.y + light.dir.1;
    if x < 0 || y < 0 || x >= map[0].len() as isize || y >= map.len() as isize {
        return Vec::new();
    }
    let mut new_lights = Vec::new();
    match map[y as usize][x as usize] {
        '.' => {
            let new_light = Light { x, y, dir: light.dir};
            new_lights.push(new_light);
        },
        '\\' => {
            let new_dir = match light.dir {
                (1, 0) => (0, 1),
                (0, -1) => (-1, 0),
                (-1, 0) => (0, -1),
                (0, 1) => (1, 0),
                _ => panic!("Invalid direction"),
            };
            let new_light = Light { x, y, dir: new_dir};
            new_lights.push(new_light);
        },
        '/' => {
            let new_dir = match light.dir {
                (1, 0) => (0, -1),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, 1),
                (0, -1) => (1, 0),
                _ => panic!("Invalid direction"),
            };
            let new_light = Light { x, y, dir: new_dir};
            new_lights.push(new_light);
        },
        '|' => {
            match light.dir {
                (1, 0) | (-1, 0) => {
                    let new_light1 = Light { x, y, dir: (0, 1)};
                    let new_light2 = Light { x, y, dir: (0, -1)};
                    new_lights.push(new_light1);
                    new_lights.push(new_light2);
                },
                (0, 1) | (0, -1) => {
                    let new_light = Light { x, y, dir: light.dir};
                    new_lights.push(new_light);
                },
                _ => panic!("Invalid direction"),
            };
        },
        '-' => {
            match light.dir {
                (0, 1) | (0, -1) => {
                    let new_light1 = Light { x, y, dir: (1, 0)};
                    let new_light2 = Light { x, y, dir: (-1, 0)};
                    new_lights.push(new_light1);
                    new_lights.push(new_light2);
                },
                (1, 0) | (-1, 0) => {
                    let new_light = Light { x, y, dir: light.dir};
                    new_lights.push(new_light);
                },
                _ => panic!("Invalid direction"),
            };
        },
        _ => panic!("Invalid character"),
    };
    new_lights
}
