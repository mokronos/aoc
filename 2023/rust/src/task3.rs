use rust::read_data;

#[allow(unused)]
const TEST_DATA: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

pub fn run3_1()
{
    let data = read_data("data/data3.txt");
    // let data = test_data;
    let mut total = 0;

    let grid = Grid::build(data);
    let mut items = Vec::new();
    for (i, row) in grid.grid.iter().enumerate() {
        items.extend(get_items(row, i));
    }

    for mut item in items {
        item.build_border(grid.height, grid.width);
        if item.is_part(&grid) {
            total += item.id;
        }
    }

    println!("Task 3_1 Solution:\nTotal: {}", total);
}

pub fn run3_2()
{
    let data = read_data("data/data3.txt");
    let mut total = 0;

    let grid = Grid::build(data);
    let mut items = Vec::new();
    for (i, row) in grid.grid.iter().enumerate() {
        items.extend(get_items(row, i));
    }

    let mut gears: Vec<Gear> = Vec::new();
    for mut item in items {
        item.build_border(grid.height, grid.width);
        let new_gear = item.get_gear(&grid);
        if new_gear.is_none() {
            continue;
        }
        let new_gear = new_gear.unwrap();
        for gear in &gears {
            if gear.pos == new_gear.pos {
                total += gear.neighbour.id * new_gear.neighbour.id;
            }
        }
        gears.push(new_gear);
    }


    println!("Task 3_2 Solution:\nTotal: {}", total);
}

#[derive(Clone, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

struct Dir {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Item {
    id: usize,
    pos: Vec<Point>,
    border: Vec<Point>,
}

struct Gear {
    pos: Point,
    neighbour: Item,
}

struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn build (data: String) -> Grid {
        let mut grid = Vec::new();
        let mut width = 0;
        let mut height = 0;

        for line in data.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            grid.push(row);
            height += 1;
            width = line.len();
        }

        Grid {
            width,
            height,
            grid,
        }
    }
}

impl Item {
    fn build_border (&mut self, height: usize, width: usize) {

        const DIRS : [Dir; 8] = [
            Dir { x: -1, y: -1 },
            Dir { x:  0, y: -1 },
            Dir { x:  1, y: -1 },
            Dir { x: -1, y:  0 },
            Dir { x:  1, y:  0 },
            Dir { x: -1, y:  1 },
            Dir { x:  0, y:  1 },
            Dir { x:  1, y:  1 },
        ];

        let mut border = Vec::new();
        for pos in &self.pos {
            for dir in &DIRS {
                let x = pos.x as i32 + dir.x;
                let y = pos.y as i32 + dir.y;
                if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
                    continue;
                }
                let border_point = Point {
                    x: x as usize,
                    y: y as usize,
                };
                if !self.pos.contains(&border_point) && !border.contains(&border_point) {
                    border.push(border_point);
                }
            }
        }
        self.border = border;
    }
}

fn get_items(row: &Vec<char>, row_id: usize) -> Vec<Item> {

    let mut items = Vec::new();
    let mut number = String::new();
    let mut pos = Vec::new();
    let mut i = 0;
    let mut c: char;

    while i <= row.len() {
        if i == row.len() {
            if !number.is_empty() {
                let id = number.parse::<usize>().unwrap();
                let item = Item {
                    id,
                    pos: pos.clone(),
                    border: Vec::new(),
                };
                items.push(item);
            }
            break;
        }

        c = row[i];
        if c.is_numeric() {
            number.push(c);
            pos.push(Point {
                x: i,
                y: row_id,
            });
        }

        if !c.is_numeric() && !number.is_empty() {
            let id = number.parse::<usize>().unwrap();
            let item = Item {
                id,
                pos: pos.clone(),
                border: Vec::new(),
            };
            items.push(item);
            number.clear();
            pos.clear();
        }
        i += 1;

    }

    items
}

impl Item {
    fn is_part(&mut self, grid: &Grid) -> bool {

        for point in &self.border {
            if grid.grid[point.y][point.x] != '.' && !grid.grid[point.y][point.x].is_numeric() {
                return true;
            }
        }
        false
    }
}

impl Item {
    fn get_gear(self, grid: &Grid) -> Option<Gear>{

        for point in &self.border {
            if grid.grid[point.y][point.x] == '*' {
                let neighbour = self.clone();
                return Some(Gear {
                    pos: point.clone(),
                    neighbour,
                });
            }
        }
        None
    }
}
