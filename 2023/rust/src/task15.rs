use rust::read_data;

#[allow(unused)]
const TEST_DATA: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

pub fn run15_1()
{
    let data = read_data("data/data15.txt");
    // let data = TEST_DATA;
    let total = solve(&data);

    println!("Task 15_1 Solution:\nTotal: {}", total);
}

pub fn run15_2()
{
    let data = read_data("data/data15.txt");
    // let data = TEST_DATA;
    let total = solve2(&data);

    println!("Task 15_2 Solution:\nTotal: {}", total);
}

fn solve(data: &str) -> usize {
    let entires: Vec<&str> = data.lines().next().unwrap().split(',').collect();
    let mut total = 0;
    for entry in entires {
        total += hash(&entry.to_string()) as usize;
    }
    total
}

fn solve2(data: &str) -> usize {
    let entires: Vec<&str> = data.lines().next().unwrap().split(',').collect();
    let mut boxes = vec![(); 256].into_iter().map(|_| Vec::new()).collect::<Vec<Vec<Lens>>>();
    for entry in entires {
        let lens = Lens::parse(entry);
        let hash = hash(&lens.label);
        let boxx = &mut boxes[hash as usize];
        let index = boxx.iter().position(|l| l.label == lens.label);
        match index {
            Some(i) => {
                if lens.operation == '=' {
                    boxx[i] = lens;
                } else {
                    boxx.remove(i);
                }
            },
            None => {
                if lens.operation == '=' {
                    boxx.push(lens);
                }
            }
        }
    }
    let mut total = 0;
    for boxx in 0..boxes.len() {
        for lens_pos in 0..boxes[boxx].len() {
            total += (boxx + 1) * (lens_pos + 1) * boxes[boxx][lens_pos].focal_length;
        }
    }
    
    total
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: usize,
    operation: char,
}

impl Lens {
    fn parse(entry: &str) -> Self {
        let mut operation = '-';
        let mut focal_length = 0;
        let mut label = "".to_string();
        if entry.chars().last().unwrap().is_digit(10) {
            focal_length = entry.chars().last().unwrap().to_digit(10).unwrap() as usize;
            operation = '=';
            label = entry[..entry.len()-2].to_string();
        } else {
            label = entry[..entry.len()-1].to_string();
        }

        Self { label, focal_length, operation }
    }
}

fn hash(data: &String) -> usize {
    let mut cur = 0;
    for c in data.chars() {
        let ascii = c as u64;
        cur = cur + ascii;
        cur *= 17;
        cur %= 256;
    }
    cur as usize
}
