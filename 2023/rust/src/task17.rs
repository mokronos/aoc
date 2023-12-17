use rust::read_data;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[allow(unused)]
const TEST_DATA: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

pub fn run17_1()
{
    let data = read_data("data/data17.txt");
    // let data = TEST_DATA;
    let total = solve(&data);

    println!("Task 17_1 Solution:\nTotal: {}", total);
}

pub fn run17_2()
{
    let data = read_data("data/data17.txt");
    // let data = TEST_DATA;
    let total = solve2(&data);

    println!("Task 17_2 Solution:\nTotal: {}", total);
}

fn solve(data: &str) -> u64
{
    let map = data.lines().map(str::as_bytes).collect::<Vec<_>>();
    let best = dijkstra(&map, 1, 3);
    best as u64
}

fn solve2(data: &str) -> u64
{
    let map = data.lines().map(str::as_bytes).collect::<Vec<_>>();
    let best = dijkstra(&map, 4, 10);
    best as u64
}


fn dijkstra(grid: &[&[u8]], minstep: isize, maxstep: isize) -> i64 {
  let mut dists = HashMap::new();
  let mut q = BinaryHeap::from_iter([(0, (0,0,(0,0)))]);
  while let Some((cost, (r, c, d))) = q.pop() {
    if (r,c) == (grid.len() - 1, grid[0].len() - 1) {
      return -cost;
    }
    if dists.get(&(r, c, d)).is_some_and(|&c| -cost > c) {
      continue;
    }
    for (dr, dc) in [(-1,0), (1,0), (0,-1), (0,1)] {
      if d == (dr, dc) || d == (-dr, -dc) {
        continue;
      }
      let mut next_cost = -cost;
      for dist in 1..=maxstep {
        let rr = (r as isize + dr * dist) as usize;
        let cc = (c as isize + dc * dist) as usize;
        if rr >= grid.len() || cc >= grid[0].len() {
          continue;
        }
        next_cost += (grid[rr][cc] - b'0') as i64;
        if dist < minstep {
          continue;
        }
        let key = (rr, cc, (dr, dc));
        if next_cost < *dists.get(&key).unwrap_or(&i64::MAX) {
          dists.insert(key, next_cost);
          q.push((-next_cost, key));
        }
      }
    }
  }
  unreachable!()
}
