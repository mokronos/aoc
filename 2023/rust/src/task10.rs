use rust::read_data;

#[allow(unused)]
const TEST_DATA: &str = "..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........";

const TEST_DATA2: &str = "...........
.S-------7.
.|F-----7|.
.||OOOOO||.
.||OOOOO||.
.|L-7OF-J|.
.|II|O|II|.
.L--JOL--J.
.....O.....";

pub fn run10_1()
{
    let data = read_data("data/data10.txt");
    // let data = TEST_DATA;
    let mut total = 0;
    let (map, start) = parse_data(&data);
    let mut dirs = get_start_dir(&map, start);

    total = find_furthest(&map, start, dirs);

    println!("Task 10_1 Solution:\nTotal: {}", total);
}

pub fn run10_2()
{
    let data = read_data("data/data10.txt");
    // let data = TEST_DATA;
    let mut total = 0;
    let (map, start) = parse_data(&data);
    let mut dirs = get_start_dir(&map, start);

    let path = find_path(&map, start, dirs);
    total = get_inside(&map, path);

    println!("Task 10_2 Solution:\nTotal: {}", total);
}

fn parse_data(data: &str) -> (Vec<Vec<char>>, (i32, i32))
{
    let mut map = Vec::new();
    let mut start = (0,0);
    for (y, line) in data.lines().enumerate()
    {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate()
        {
            if c == 'S' {
                start = (x as i32, y as i32);
            }
            row.push(c);
        }
        map.push(row);
    }
    (map, start)
}

fn get_start_dir(map: &Vec<Vec<char>>, start: (i32, i32)) -> Vec<(i32, i32)>
{
    let mut dirs = Vec::new();
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    let right = (start.0 + 1, start.1);
    let left = (start.0 - 1, start.1);
    let up = (start.0, start.1 - 1);
    let down = (start.0, start.1 + 1);

    if right.0 < width && "-7J".contains(map[right.1 as usize][right.0 as usize]) {
        dirs.push((1, 0));
    }
    if left.0 >= 0 && "-FL".contains(map[left.1 as usize][left.0 as usize]) {
        dirs.push((-1, 0));
    }
    if up.1 >= 0 && "|F7".contains(map[up.1 as usize][up.0 as usize]) {
        dirs.push((0, -1));
    }
    if down.1 < height && "|JL".contains(map[down.1 as usize][down.0 as usize]) {
        dirs.push((0, 1));
    }
    dirs
}

fn find_furthest(map: &Vec<Vec<char>>, start: (i32, i32), dirs: Vec<(i32, i32)>) -> i32
{
    let mut cur1 = (start.0, start.1);
    let mut cur2 = (start.0, start.1);
    let mut dir1 = dirs[0];
    let mut dir2 = dirs[1];
    let mut count = 0;

    loop {
        (cur1, dir1) = step(&map, cur1, dir1);
        (cur2, dir2) = step(&map, cur2, dir2);
        count += 1;
        if cur1 == cur2 {
            break;
        }
    }
    count
}

fn step(map: &Vec<Vec<char>>, pos: (i32, i32), dir: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
    let new_dir = match map[new_pos.1 as usize][new_pos.0 as usize] {
        // just keep going that direction
        '-' => dir,
        '|' => dir,
        // change direction
        '7' => if dir == (1, 0) { (0, 1) } else { (-1, 0) },
        'J' => if dir == (1, 0) { (0, -1) } else { (-1, 0) },
        'F' => if dir == (-1, 0) { (0, 1) } else { (1, 0) },
        'L' => if dir == (-1, 0) { (0, -1) } else { (1, 0) },
        _ => (0, 0),
    };
    (new_pos, new_dir)
}

fn find_path(map: &Vec<Vec<char>>, start: (i32, i32), dirs: Vec<(i32, i32)>) -> Vec<(i32, i32)>
{
    let mut cur1 = (start.0, start.1);
    let mut dir1 = dirs[0];
    let mut path = Vec::new();
    let target = (start.0, start.1);
    path.push(cur1);

    loop {
        (cur1, dir1) = step(&map, cur1, dir1);
        path.push(cur1);
        if cur1 == target {
            break;
        }
    }
    path
}

fn paint_outside(map: &Vec<Vec<char>>, start: (i32, i32), dirs: Vec<(i32, i32)>) -> Vec<(i32, i32)>
{
    let mut cur1 = (start.0, start.1);
    let mut dir1 = dirs[0];
    let mut old_side= dirs[0];
    let mut outside = Vec::new();
    let target = (start.0, start.1);

    loop {
        let side = match dir1 {
            (1, 0) => (0, -1),
            (-1, 0) => (0, 1),
            (0, 1) => (1, 0),
            (0, -1) => (-1, 0),
            _ => (0, 0),
        };
        let side_pos = (cur1.0 + side.0, cur1.1 + side.1);
        let old_side_pos = (cur1.0 + old_side.0, cur1.1 + old_side.1);
        old_side = side;
        if side_pos.0 > 0 && side_pos.1 > 0 && side_pos.0 < map[0].len() as i32 && side_pos.1 < map.len() as i32 {
            if map[side_pos.1 as usize][side_pos.0 as usize] == '.' {
                outside.push(side_pos);
            }
        }
        if old_side_pos.0 < 0 || old_side_pos.1 < 0 || old_side_pos.0 >= map[0].len() as i32 || old_side_pos.1 >= map.len() as i32 {
        } else {
            if map[old_side_pos.1 as usize][old_side_pos.0 as usize] == '.' {
                outside.push(old_side_pos);
            }
        }

        (cur1, dir1) = step(&map, cur1, dir1);
        if cur1 == target {
            break;
        }
    }
    let mut map = map.clone();
    for pos in outside.iter() {
        map[pos.1 as usize][pos.0 as usize] = 'O';
    }
    outside
}

fn get_inside(map: &Vec<Vec<char>>, path: Vec<(i32, i32)>) -> i32
{
    let mut inside: Vec<(i32, i32)> = Vec::new();
    let mut outside: Vec<(i32, i32)> = Vec::new();
    let mut map = clean_map(map, &path);

    // add borders of map to outside
    for y in 0..map.len()+2 {
        outside.push((-1, y as i32 -1));
        outside.push((map[0].len() as i32, y as i32 -1));
    }
    for x in 0..map[0].len()+2 {
        outside.push((x as i32 - 1, -1 as i32));
        outside.push((x as i32 - 1, map.len() as i32));
    }
    outside.extend(paint_outside(&map, path[0], get_start_dir(&map, path[0])));

    for x in 0..map[0].len() {
        for y in 0..map.len() {
            let cur = (x as i32, y as i32);
            let mut queue = Vec::new();
            let mut visited = Vec::new();
            let mut found_inside = true;
            const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            queue.push(cur);
            while queue.len() > 0 {
                let cur = queue.remove(0);
                if visited.contains(&cur) || path.contains(&cur) {
                    continue;
                }
                if inside.contains(&cur) {
                    break;
                }
                if outside.contains(&cur) {
                    found_inside = false;
                    break;
                }
                visited.push(cur);
                for dir in DIRS.iter() {
                    let new_pos = (cur.0 + dir.0, cur.1 + dir.1);
                    queue.push(new_pos);
                }
            }
            if found_inside {
                inside.extend(visited.clone());
            } else {
                outside.extend(visited.clone());
            }
    }
    }
    inside.len() as i32
}


fn clean_map(map: &Vec<Vec<char>>, path: &Vec<(i32, i32)>) -> Vec<Vec<char>>
{
    let mut map = map.clone();
    for x in 0..map[0].len() {
        for y in 0..map.len() {
            let cur = (x as i32, y as i32);
            if !path.contains(&cur) {
                map[y][x] = '.';
            }
        }
    }
    map
}
