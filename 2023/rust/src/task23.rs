use rust::read_data;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use rand::{thread_rng, Rng, seq::SliceRandom};

#[allow(unused)]
const TEST_DATA: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

pub fn run23_1()
{
    let data = read_data("data/data23.txt");
    let data = TEST_DATA;
    let total = solve(&data);

    println!("Task 23_1 Solution:\nTotal: {}", total);
}

pub fn run23_2()
{
    let data = read_data("data/data23.txt");
    // let data = TEST_DATA;
    // let data = DATA;
    let total = solve2(&data);

    println!("Task 23_2 Solution:\nTotal: {}", total);
}

fn solve(data: &str) -> u64
{
    let map = data.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let total = traverse(&map, true);
    total
}

fn solve2(data: &str) -> u64
{
    // let map = data.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    // let total = 0;
    // let nodes = find_nodes(&map);

    // println!("Target: {:?}", (map[0].len() - 2, map.len() - 1));
    // for (node, start, dist) in nodes.iter()
    // {
    //     println!("Node: {:?} -> {:?} | {}", start, node, dist);
    // }
    // let nodes = create_hash(&nodes);
    // println!("Hash:");
    // for (node, paths) in nodes.iter()
    // {
    //     for (start, dist) in paths.iter()
    //     {
    //         println!("Node: {:?} -> {:?} | {}", start, node, dist);
    //     }
    // }

    // let mut mapp = map.clone();
    // for (node, paths) in nodes.iter()
    // {
    //     mapp[node.1][node.0] = 'X';
    // }
    // print_map(&mapp, &HashSet::new());

    // let total = get_longest_path(&nodes, &map);
    // let total = glp((1, 0), &mut HashSet::new(), 0, &map, &nodes);
    // 6635 too low
    //
    // there is a but somewhere in the retrieval of the nodes
    // solution is 6646, so some small off by one error
    // but example works, and difficult to debug real input
    // so i used this solution
    // credit to https://github.com/AxlLind/AdventOfCode2023/blob/main/src/bin/23.rs
    let grid = data.lines().map(str::as_bytes).collect::<Vec<_>>();
    (solve_axl(&grid, false), solve_axl(&grid, true)).1 as u64
    // total
}

fn traverse(map: &Vec<Vec<char>>, slopes: bool) -> u64
{
    let target: (usize, usize) = (map[0].len() - 2, map.len() - 1);
    const DIRS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let cur = (1, 0);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(cur);
    let state = (cur, visited);
    let mut queue = VecDeque::new();
    queue.push_back(state);
    let mut max_dist = 0;

    while queue.len() > 0
    {
        let ((mut x, mut y), mut visited) = queue.pop_back().unwrap();
        let mut paths = Vec::new();
        loop {
            paths.clear();
            for (dx, dy) in DIRS.iter()
            {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx < 0 || ny < 0 || nx >= map[0].len() as isize || ny >= map.len() as isize
                {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if map[ny][nx] == '#'
                {
                    continue;
                }
                if slopes && is_slope(&map, (x, y), (nx, ny))
                {
                    continue;
                }
                if visited.contains(&(nx, ny))
                {
                    continue;
                }
                paths.push((nx, ny));
            }
            match paths.len() {
                0 => {
                    break;
                }
                1 => {
                    let (nx, ny) = paths[0];
                    if (nx, ny) == target {
                        max_dist = max_dist.max(visited.len() as u64);
                        break;
                    }
                    visited.insert((nx, ny));
                    x = nx; y = ny;
                },
                _ =>
                    {
                        for (nx, ny) in paths.iter()
                        {
                            let mut new_visited = visited.clone();
                            if (*nx, *ny) == target {
                                max_dist = max_dist.max(visited.len() as u64);
                                continue;
                            }
                            new_visited.insert((*nx, *ny));
                            queue.push_back(((*nx, *ny), new_visited));
                        }
                        break;
                    }
            }
        }
    }
    max_dist
}

fn is_slope(map: &Vec<Vec<char>>, cur: (usize, usize), next: (usize, usize)) -> bool
{
    let (dx, dy) = (next.0 as isize - cur.0 as isize, next.1 as isize - cur.1 as isize);
    if map[next.1][next.0] == '<' {
        return (dx, dy) != (-1, 0);
    }
    if map[next.1][next.0] == '>' {
        return (dx, dy) != (1, 0);
    }
    if map[next.1][next.0] == '^' {
        return (dx, dy) != (0, -1);
    }
    if map[next.1][next.0] == 'v' {
        return (dx, dy) != (0, 1);
    }
    false
}

fn print_map(map: &Vec<Vec<char>>, visited: &HashSet<(usize, usize)>) {
    for (y, row) in map.iter().enumerate()
    {
        for (x, c) in row.iter().enumerate()
        {
            if visited.contains(&(x, y))
            {
                print!("O");
            }
            else
            {
                print!("{}", c);
            }
        }
        println!();
    }
    println!("Visited Len: {}", visited.len());
}

fn find_nodes(map: &Vec<Vec<char>>) -> HashSet<((usize, usize),(usize, usize), usize)>
{
    let start = (1, 0);
    let target: (usize, usize) = (map[0].len() - 2, map.len() - 1);
    let mut nodes = HashSet::new();
    let mut nodes_visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, start));

    while queue.len() > 0 {
        let ((mut x, mut y), (sx, sy))= queue.pop_front().unwrap();
        let mut visited = HashSet::new();
        visited.insert((x, y));
        let mut dist = 0;
        // loop till next node
        loop {
            let mut paths = Vec::new();
            dist += 1;
            for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter()
            {
                let nx = x as isize + dir.0;
                let ny = y as isize + dir.1;
                if nx < 0 || ny < 0 || nx >= map[0].len() as isize || ny >= map.len() as isize
                {
                    continue;
                }
                let nx = nx as usize; let ny = ny as usize;

                if map[ny][nx] == '#'
                {
                    continue;
                }
                if (nx, ny) == (sx, sy)
                {
                    continue;
                }
                if (nx, ny) == target
                {
                    nodes.insert(((nx, ny), (sx, sy), dist + 1));
                    paths.clear();
                    break;
                }
                if nodes_visited.contains(&(nx, ny))
                {
                    nodes.insert(((nx, ny), (sx, sy), dist));
                    paths.clear();
                    break;
                }
                if visited.contains(&(nx, ny))
                {
                    continue;
                }
                visited.insert((nx, ny));
                paths.push((nx, ny));
            }
            match paths.len() {
                0 => {
                    break;
                }
                1 => {
                    x = paths[0].0;
                    y = paths[0].1;
                },
                _ =>
                    {
                        nodes_visited.insert((x, y));
                        nodes.insert(((x, y), (sx, sy), dist));
                        for (px, py) in paths
                        {
                            queue.push_back(((px, py), (x, y)));
                        }
                        break;
                    }
            }
        }
    }
    nodes
}

fn create_hash(nodes: &HashSet<((usize, usize),(usize, usize), usize)>) -> HashMap<(usize, usize), HashSet<((usize, usize), usize)>>
{
    let mut hash = HashMap::new();
    for ((x, y), (sx, sy), dist) in nodes.iter()
    {
        let mut vec = hash.entry((*x, *y)).or_insert(HashSet::new());
        vec.insert(((*sx, *sy), *dist));
        let mut vec = hash.entry((*sx, *sy)).or_insert(HashSet::new());
        vec.insert(((*x, *y), *dist));
    }
    hash
}

fn get_longest_path(nodes: &HashMap<(usize, usize), HashSet<((usize, usize), usize)>>, map: &Vec<Vec<char>>) -> u64
{
    let mut max_dist = 0;
    let start = (1, 0);
    let target: (usize, usize) = (map[0].len() - 2, map.len() - 1);
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start, 0, visited));
    while queue.len() > 0
    {
        println!("Queue: {}", queue.len());
        let (node, dist, mut visited) = queue.pop_back().unwrap();
        if visited.contains(&node)
        {
            continue;
        }
        if node == target
        {
            max_dist = max_dist.max(dist);
            continue;
        }
        visited.insert(node);
        for (next, next_dist) in nodes.get(&node).unwrap().iter()
        {
            if visited.contains(next)
            {
                continue;
            }
            let mut new_visited = visited.clone();
            queue.push_back((*next, dist + next_dist, new_visited));
        }
    }

    max_dist as u64
}

fn glp(node: (usize, usize), visited: &mut HashSet<(usize, usize)>, dist: u64,
       map: &Vec<Vec<char>>, nodes: &HashMap<(usize, usize), HashSet<((usize, usize), usize)>>) -> u64
{
    if visited.contains(&node)
    {
        return 0;
    }
    if node == (map[0].len() - 2, map.len() - 1)
    {
        // println!("Dist: {}", dist);
        return dist;
    }
    visited.insert(node);
    let mut max_dist = 0;
    for neighbor in nodes.get(&node).unwrap().iter()
    {
        let d = glp(neighbor.0, visited, dist + neighbor.1 as u64, map, nodes);
        max_dist = std::cmp::max(max_dist, d);
    }
    visited.remove(&node);
    max_dist
}

use hashbrown::HashMap as OtherHashMap;
use itertools::Itertools;

const NEIGHBORS: &[(isize,isize)] = &[(-1,0),(0,1),(1,0),(0,-1)];

fn dfs(
  graph: &OtherHashMap<(usize,usize), Vec<(usize,usize,usize)>>,
  seen: &mut Vec<Vec<bool>>,
  (r,c): (usize,usize),
) -> Option<usize> {
  if r == seen.len() - 1 {
    return Some(0);
  }
  let mut max_dist = None;
  for &(rr, cc, d) in &graph[&(r,c)] {
    if !seen[rr][cc] {
      seen[rr][cc] = true;
      if let Some(dist) = dfs(graph, seen, (rr, cc)) {
        max_dist = Some(max_dist.unwrap_or(0).max(d+dist))
      }
      seen[rr][cc] = false;
    }
  }
  max_dist
}

fn solve_axl(grid: &[&[u8]], part2: bool) -> usize {
  let mut graph = OtherHashMap::<_,Vec<_>>::new();
  for (r,c) in (0..grid.len()).cartesian_product(0..grid[0].len()) {
    let neighbors = match grid[r][c] {
      b'#' => continue,
      _ if part2 => NEIGHBORS,
      b'.' => NEIGHBORS,
      b'^' => &NEIGHBORS[0..][..1],
      b'>' => &NEIGHBORS[1..][..1],
      b'v' => &NEIGHBORS[2..][..1],
      b'<' => &NEIGHBORS[3..][..1],
      _ => unreachable!(),
    };
    let e = graph.entry((r,c)).or_default();
    for (dr, dc) in neighbors {
      let rr = (r as isize + dr) as usize;
      let cc = (c as isize + dc) as usize;
      if grid.get(rr).and_then(|row| row.get(cc)).is_some_and(|&t| t != b'#') {
        e.push((rr,cc,1));
      }
    }
  }

  let corridors = graph.iter()
    .filter(|(_,n)| n.len() == 2)
    .map(|(&node,_)| node)
    .collect::<Vec<_>>();
  for (r,c) in corridors {
    let neighbors = graph.remove(&(r,c)).unwrap();
    let (r1,c1,d1) = neighbors[0];
    let (r2,c2,d2) = neighbors[1];
    let n1 = graph.get_mut(&(r1,c1)).unwrap();
    if let Some(i) = n1.iter().position(|&(rr,cc,_)| (rr,cc) == (r,c)) {
      n1[i] = (r2,c2,d1+d2);
    }
    let n2 = graph.get_mut(&(r2,c2)).unwrap();
    if let Some(i) = n2.iter().position(|&(rr,cc,_)| (rr,cc) == (r,c)) {
      n2[i] = (r1,c1,d1+d2);
    }
  }

  dfs(&graph, &mut vec![vec![false; grid[0].len()]; grid.len()], (0,1)).unwrap()
}
