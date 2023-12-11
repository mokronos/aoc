use rust::read_data;

#[allow(unused)]
const TEST_DATA: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

pub fn run11_1()
{
    let data = read_data("data/data11.txt");
    // let data = TEST_DATA;
    let mut total = 0;
    let map = parse_data(&data);
    let map = alter_data(map, 1);
    let galaxies = get_galaxies(&map);
    for (i, (x1,y1)) in galaxies.iter().enumerate()
    {
        for (j, (x2,y2)) in galaxies[i..].iter().enumerate()
        {
            let dx = *x2 as i32 -*x1 as i32;
            let dy = *y2 as i32 -*y1 as i32;
            let dist = dx.abs() + dy.abs();
            total += dist;
        }
    }


    println!("Task 11_1 Solution:\nTotal: {}", total);
}

pub fn run11_2()
{
    let data = read_data("data/data11.txt");
    // let data = TEST_DATA;
    let mut total = 0;
    let map = parse_data(&data);
    let galaxies = get_galaxies(&map);
    let spaces = get_spaces(&map);
    const DIST: i32 = 1_000_000 - 1;
    // const DIST: i32 = 100 - 1;
    // const DIST: i32 = 2 - 1;
    for (i, (x1,y1)) in galaxies.iter().enumerate()
    {
        for (j, (x2,y2)) in galaxies[i+1..].iter().enumerate()
        {
            let dx = *x2 as i32 -*x1 as i32;
            let dxspace: i32 = count_inbetween(*x1, *x2, &spaces.x) as i32 * DIST;
            let dy = *y2 as i32 -*y1 as i32;
            let dyspace: i32 = count_inbetween(*y1, *y2, &spaces.y) as i32 * DIST;
            let dist = dx.abs() + dy.abs() + dxspace + dyspace;
            total += dist as u64;
        }
    }

    println!("Task 11_2 Solution:\nTotal: {}", total);
}

#[derive(Debug)]
struct Spaces
{
    x: Vec<usize>,
    y: Vec<usize>,
}

fn parse_data(data: &str) -> Vec<Vec<char>>
{
    let mut map = Vec::new();
    for line in data.lines()
    {
        let mut row = Vec::new();
        for c in line.chars()
        {
            row.push(c);
        }
        map.push(row);
    }
    map
}

fn alter_data(data: Vec<Vec<char>>, dist: usize) -> Vec<Vec<char>>
{
    let mut map = Vec::new();
    let mut number = 1;
    let mut cols = Vec::new();
    for i in 0..data[0].len()
    {
        let mut check = true;
        for j in 0..data.len()
        {
            if data[j][i] != '.'
            {
                check = false;
                break;
            }
        }
        if check
        {
            cols.push(i);
        }
    }
    let ext = vec!['.'; cols.len()*dist];

    for line in data.iter()
    {
        if line.iter().all(|&c| c == '.')
        {
            let mut cur_line = line.clone();
            cur_line.extend(ext.clone());
            map.push(cur_line.clone());
            for _ in 0..dist
            {
                map.push(cur_line.clone());
            }
        } else {
            let mut row = Vec::new();
            for (i,c) in line.iter().enumerate() {

                if *c == '.' {
                    row.push('.');
                    if cols.contains(&i) {
                        for _ in 0..dist
                        {
                            row.push('.');
                        }
                    }
                } else {
                    row.push(number.to_string().chars().next().unwrap());
                    number += 1;
                }
            }
            map.push(row);
    }
    }
    
    map
}

fn get_galaxies(map: &Vec<Vec<char>>) -> Vec<(usize, usize)>
{
    let mut galaxies = Vec::new();
    for (i, line) in map.iter().enumerate()
    {
        for (j, c) in line.iter().enumerate()
        {
            if *c != '.'
            {
                galaxies.push((j,i));
            }
        }
    }
    galaxies
}

fn get_spaces(map: &Vec<Vec<char>>) -> Spaces
{
    let mut spaces = Spaces{x: Vec::new(), y: Vec::new()};
    for (i, line) in map.iter().enumerate()
    {
        if line.iter().all(|&c| c == '.')
        {
            spaces.y.push(i);
        }
        let mut check = true;
        for j in 0..map.len()
        {
            if map[j][i] != '.'
            {
                check = false;
                break;
            }
        }
        if check
        {
            spaces.x.push(i);
        }
    }
    spaces
}

fn count_inbetween(a: usize, b: usize, spaces: &Vec<usize>) -> usize
{
    let mut count = 0;
    let mut start = a;
    let mut end = b;
    if a == b
    {
        return 0;
    }
    if a > b
    {
        start = b;
        end = a;
    }
    for i in start+1..end
    {
        if spaces.contains(&i)
        {
            count += 1;
        }
    }
    count
}
