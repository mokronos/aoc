use rust::read_data;

#[allow(unused)]
const TEST_DATA: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

pub fn run14_1()
{
    let data = read_data("data/data14.txt");
    // let data = TEST_DATA;
    let total = solve(&data);

    println!("Task 14_1 Solution:\nTotal: {}", total);
}

pub fn run14_2()
{
    let data = read_data("data/data14.txt");
    // let data = TEST_DATA;
    let total = solve2(&data);

    println!("Task 14_2 Solution:\nTotal: {}", total);
}


fn parse(data: &str) -> Vec<Vec<char>>
{
    let map: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    map
}

fn print(map: &Vec<Vec<char>>) {
    println!("{}", map.iter().map(|line| line.iter().collect::<String>()).collect::<Vec<String>>().join("\n"));
}

fn solve(data: &str) -> usize {
    let mut map = parse(data);
    // let tilted_map = tilt(&mut map, 'N');
    tilt(&mut map, 'N');
    load(&map)
}

fn solve2(data: &str) -> usize {
    let mut map = parse(data);
    let mut history = Vec::new();
    history.push(map.clone());

    const CYCLES: usize = 1_000_000_000;
    let mut i = 0;
    while i < CYCLES {
        cycle(&mut map);
        if history.contains(&map) {
            let index = history.iter().position(|x| *x == map).unwrap();
            let cycle_length = history.len() - index;
            let cycles_left = (CYCLES - i - 1) % cycle_length;
            for _ in 0..cycles_left {
                cycle(&mut map);
            }
            break;
        }
        history.push(map.clone());
        i += 1;
    }
    load(&map)
}

fn load(map: &Vec<Vec<char>>) -> usize {
    let mut load = 0;
    for y in 0..map.len() {
        load += map[y].iter().filter(|&&c| c == 'O').count() * (map.len() - y);
    }
    load
}

fn cycle(map: &mut Vec<Vec<char>>) {
    for dir in ['N', 'W', 'S', 'E'] {
        tilt(map, dir);
    }
}

fn tilt(map: &mut Vec<Vec<char>>, dir: char) {
    let height = map.len();
    let width = map[0].len();
    match dir {
        'N' => {
            for y in 0..height {
                for x in 0..width {
                    if map[y][x] == 'O' {
                        map[y][x] = '.';
                        let mut placed = false;
                        for i in (0..y).rev() {
                            if map[i][x] == '#' || map[i][x] == 'O' {
                                map[i+1][x] = 'O';
                                placed = true;
                                break;
                            }
                        }
                        if !placed {
                            map[0][x] = 'O';
                        }
                    }
                }
            }
        },
        'W' => {
            for x in 0..height {
                for y in 0..height {
                    if map[y][x] == 'O' {
                        map[y][x] = '.';
                        let mut placed = false;
                        for i in (0..x).rev() {
                            if map[y][i] == '#' || map[y][i] == 'O' {
                                map[y][i+1] = 'O';
                                placed = true;
                                break;
                            }
                        }
                        if !placed {
                            map[y][0] = 'O';
                        }
                    }
                }
            }
        },
        'S' => {
            for y in (0..height).rev() {
                for x in 0..width {
                    if map[y][x] == 'O' {
                        map[y][x] = '.';
                        let mut placed = false;
                        for i in y..height {
                            if map[i][x] == '#' || map[i][x] == 'O' {
                                map[i-1][x] = 'O';
                                placed = true;
                                break;
                            }
                        }
                        if !placed {
                            map[height-1][x] = 'O';
                        }
                    }
                }
            }
        },
        'E' => {
            for x in (0..height).rev() {
                for y in 0..width {
                    if map[y][x] == 'O' {
                        map[y][x] = '.';
                        let mut placed = false;
                        for i in x..width {
                            if map[y][i] == '#' || map[y][i] == 'O' {
                                map[y][i-1] = 'O';
                                placed = true;
                                break;
                            }
                        }
                        if !placed {
                            map[y][width-1] = 'O';
                        }
                    }
                }
            }
        },
        _ => panic!("Invalid direction"),
    }
}
