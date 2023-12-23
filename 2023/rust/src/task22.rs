use rust::read_data;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use rand::{thread_rng, Rng, seq::SliceRandom};

#[allow(unused)]
const TEST_DATA: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

pub fn run22_1()
{
    let data = read_data("data/data22.txt");
    // let data = TEST_DATA;
    let total = solve(&data);

    println!("Task 22_1 Solution:\nTotal: {}", total);
}

pub fn run22_2()
{
    let data = read_data("data/data22.txt");
    // let data = TEST_DATA;
    let total = solve2(&data);

    println!("Task 22_2 Solution:\nTotal: {}", total);
}

fn solve(data: &str) -> u64
{
    let mut bricks = parse_bricks(data);
    sort_z(&mut bricks);
    bricks.sort_by(|a, b| a.z.0.cmp(&b.z.0));
    let (maxx, maxy, maxz) = get_bounds(&bricks);
    let mut grid = vec![vec![vec![0; maxx+1]; maxy+1]; maxz+1];
    grid = populate_grid(&bricks, grid);
    for mut brick in bricks[..].iter_mut()
    {
        drop_brick(&mut brick, &mut grid);
    }
    let (supports, weights) = get_support(&bricks, &grid);
    let mut count = 0;
    for brick in bricks
    {
        if disintegradable(&brick, &supports, &weights)
        {
            count += 1;
        }
    }
    // print_grid(&grid);
    count as u64
}

fn solve2(data: &str) -> u64
{
    let mut bricks = parse_bricks(data);
    sort_z(&mut bricks);
    bricks.sort_by(|a, b| a.z.0.cmp(&b.z.0));
    let (maxx, maxy, maxz) = get_bounds(&bricks);
    let mut grid = vec![vec![vec![0; maxx+1]; maxy+1]; maxz+1];
    grid = populate_grid(&bricks, grid);
    for mut brick in bricks[..].iter_mut()
    {
        drop_brick(&mut brick, &mut grid);
    }
    let (supports, weights) = get_support(&bricks, &grid);
    let mut count = 0;
    for brick in bricks
    {
        count += fall_weight(&brick, &supports, &weights);
    }
    // print_grid(&grid);
    count as u64
}

#[derive(Debug)]
struct Brick
{
    id: usize,
    x: (usize, usize),
    y: (usize, usize),
    z: (usize, usize),
}

fn parse_bricks(data: &str) -> Vec<Brick>
{
    let mut bricks = Vec::new();
    for (id, line) in data.lines().enumerate()
    {
        let mut parts = line.split('~');
        let split = parts.next().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let (x1, y1, z1) = (split[0], split[1], split[2]);
        let split = parts.next().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let (x2, y2, z2) = (split[0], split[1], split[2]);
        bricks.push(Brick { id: id+1, x: (x1, x2), y: (y1, y2), z: (z1, z2) });
    }
    bricks
}

fn print_grid(grid: &Vec<Vec<Vec<usize>>>) {
    for line in grid
    {
        for row in line
        {
            println!("{:?}", row);
        }
        println!();
    }
}

fn get_bounds(bricks: &Vec<Brick>) -> (usize, usize, usize)
{
    let mut maxx = 0;
    let mut maxy = 0;
    let mut maxz = 0;
    for brick in bricks
    {
        maxx = maxx.max(brick.x.0).max(brick.x.1);
        maxy = maxy.max(brick.y.0).max(brick.y.1);
        maxz = maxz.max(brick.z.0).max(brick.z.1);
    }
    (maxx, maxy, maxz)
}

fn populate_grid(bricks: &Vec<Brick>, mut grid: Vec<Vec<Vec<usize>>>) -> Vec<Vec<Vec<usize>>>
{
    for brick in bricks
    {
        for x in brick.x.0..=brick.x.1
        {
            for y in brick.y.0..=brick.y.1
            {
                for z in brick.z.0..=brick.z.1
                {
                    grid[z][y][x] = brick.id;
                }
            }
        }
    }
    grid
}

fn sort_z(bricks: &mut Vec<Brick>)
{
    for brick in bricks
    {
        if brick.z.0 > brick.z.1
        {
            let temp = brick.z.0;
            brick.z.0 = brick.z.1;
            brick.z.1 = temp;
        }
    }
}

fn drop_brick(brick: &mut Brick, grid: &mut Vec<Vec<Vec<usize>>>)
{
    loop {
        let mut all = true;
        if brick.z.0 == 1
        {
            break;
        }
        for x in brick.x.0..=brick.x.1
        {
            for y in brick.y.0..=brick.y.1
            {
                if grid[brick.z.0-1][y][x] != 0
                {
                    all = false;
                    break;
                }
            }
        }
        if all
        {
            for x in brick.x.0..=brick.x.1
            {
                for y in brick.y.0..=brick.y.1
                {
                    grid[brick.z.1][y][x] = 0;
                    grid[brick.z.0-1][y][x] = brick.id;
                }
            }
            brick.z.0 -= 1; brick.z.1 -= 1;
        }
        else
        {
            break;
        }
    }
}

fn get_support(bricks: &Vec<Brick>, grid: &Vec<Vec<Vec<usize>>>) -> (HashMap<usize, HashSet<usize>>, HashMap<usize, HashSet<usize>>)
{
    let mut support = HashMap::new();
    let mut weight = HashMap::new();
    for brick in bricks
    {
        let mut support_set = HashSet::new();
        let mut weight_set = HashSet::new();
        for x in brick.x.0..=brick.x.1
        {
            for y in brick.y.0..=brick.y.1
            {
                if grid[brick.z.0-1][y][x] != 0
                {
                    support_set.insert(grid[brick.z.0-1][y][x]);
                }
                if grid[brick.z.1+1][y][x] != 0
                {
                    weight_set.insert(grid[brick.z.1+1][y][x]);
                }
            }
        }
        support.insert(brick.id, support_set);
        weight.insert(brick.id, weight_set);
    }
    (support, weight)
}

fn disintegradable(brick: &Brick, support: &HashMap<usize, HashSet<usize>>, weight: &HashMap<usize, HashSet<usize>>) -> bool
{
    let ws = weight.get(&brick.id);
    if ws.is_none()
    {
        return true
    }

    let ws = ws.unwrap();
    for w in ws
    {
        if support.get(w).unwrap().len() == 1
        {
            return false
        }
    }
    true
}

fn fall_weight(brick: &Brick, support: &HashMap<usize, HashSet<usize>>, weight: &HashMap<usize, HashSet<usize>>) -> usize
{
    let mut falling = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back(brick.id);
    falling.insert(brick.id);
    let mut visited = HashSet::new();
    while q.len() > 0 {
        let id = q.pop_front().unwrap();
        if visited.contains(&id) {
            continue;
        }
        visited.insert(id);
        let ws = weight.get(&id);
        if ws.is_none()
        {
            continue;
        }
        let ws = ws.unwrap();
        for w in ws
        {
            let mut all = true;
            for s in support.get(w).unwrap()
            {
                if !falling.contains(s) {
                    all = false;
                    break;
                }
            }
            if all {
                falling.insert(*w);
                q.push_back(*w);
            }
        }
    }
    falling.len() - 1
}
