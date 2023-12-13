use rust::read_data;

#[allow(unused)]
const TEST_DATA: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

pub fn run13_1()
{
    let data = read_data("data/data13.txt");
    // let data = TEST_DATA;
    let total = data.split("\n\n").map(|s| Sample::parse(s).solve()).sum::<usize>();

    // let total = 0;
    println!("Task 13_1 Solution:\nTotal: {}", total);
}

pub fn run13_2()
{
    let data = read_data("data/data13.txt");
    // let data = TEST_DATA;
    let total = data.split("\n\n").map(|s| Sample::parse(s).solve2()).sum::<usize>();

    println!("Task 13_2 Solution:\nTotal: {}", total);
}

struct Sample
{
    normal: Vec<Vec<char>>,
    flipped: Vec<Vec<char>>,
}

impl Sample
{
    fn parse(data: &str) -> Self {
        let normal = data.lines().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let flipped = flip(&normal);
        {
            Self { normal, flipped }
        }
    }

    fn solve(&self) -> usize {
        let (hor, ver) = (check(&self.normal), check(&self.flipped));
        if hor > ver { hor*100 } else { ver }
    }
    fn solve2(&self) -> usize {
        let (hor, ver) = (check(&self.normal), check(&self.flipped));
        let (new_hor, new_ver) = (check_alter(&self.normal, hor), check_alter(&self.flipped, ver));
        if new_hor > new_ver { new_hor*100 } else { new_ver }
    }

}

fn flip(data: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for i in 0..data[0].len() {
        let mut row = Vec::new();
        for j in 0..data.len() {
            row.push(data[j][i]);
        }
        result.push(row);
    }
    result
}

fn check(data: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    for start in 0..data.len()-1 {
        let outwards = std::cmp::min(start, data.len()-start-2);
        let mut check = true;
        for i in 0..outwards+1 {
            if data[start-i] != data[start+i+1] {
                check = false;
                break;
            }
        }
        if check { result = start+1; }
    }
    result
}

fn check_alter(data: &Vec<Vec<char>>, old: usize) -> usize {
    let mut result = 0;
    for k in 0..data.len() {
        for l in 0..data[0].len() {
            let mut new_data = data.clone();
            new_data[k][l] = if new_data[k][l] == '#' { '.' } else { '#' };
            for start in 0..new_data.len()-1 {
                let outwards = std::cmp::min(start, new_data.len()-start-2);
                let mut check = true;
                for i in 0..outwards+1 {
                    if new_data[start-i] != new_data[start+i+1] {
                        check = false;
                        break;
                    }
                }
                if check && start+1!=old { result = start+1; }
            }
        }
    }
    result
}
