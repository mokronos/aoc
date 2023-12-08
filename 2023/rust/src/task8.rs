use rust::read_data;
use std::collections::HashMap;

#[allow(unused)]
const TEST_DATA: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

const TEST_DATA2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

const TEST_DATA3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

pub fn run8_1()
{
    let data = read_data("data/data8.txt");
    // let data = TEST_DATA2;
    let mut total = 0;
    let data = parse_data(&data);

    let mut pos = "AAA".to_string();
    let total = travel(&data, pos, false);

    println!("Task 8_1 Solution:\nTotal: {}", total);
}

pub fn run8_2()
{
    let data = read_data("data/data8.txt");
    // let data = TEST_DATA3;
    let mut total = 0;
    let data = parse_data(&data);

    // lcm
    let total = travel_parallel(&data);

    println!("Task 8_2 Solution:\nTotal: {}", total);
}

#[derive(Debug)]
struct Data
{
    dirs: String,
    map: HashMap<String, Fork>,
}

#[derive(Debug)]
struct Fork
{
    left: String,
    right: String,
}

struct State
{
    nodes: Vec<String>,
    count: u64,
}

impl State {
    fn init(data: &Data) -> State
    {
        let mut nodes = Vec::new();
        for (key, _) in data.map.iter()
        {
            if key.ends_with("A")
            {
                nodes.push(key.clone());
            }
        }
        State{nodes: nodes, count: 0}
    }

    fn check_end(&self) -> bool
    {
        self.nodes.iter().all(|x| x.ends_with("Z"))
    }
}


fn parse_data(data: &str) -> Data
{
    let dirs = data.split("\n").next().unwrap().to_string();
    let mut map = HashMap::new();
    let data = data.split("\n\n").last().unwrap().to_string();

    for line in data.lines()
    {
        let mut line = line.split(" = ");
        let key = line.next().unwrap().to_string();
        let mut line = line.next().unwrap().split(", ");
        let left = line.next().unwrap().to_string().trim_matches('(').to_string();
        let right = line.next().unwrap().to_string().trim_matches(')').to_string();
        map.insert(key, Fork{left, right});
    }
    Data{dirs, map}
}

fn travel_parallel(data: &Data) -> u64 {

    let mut states = State::init(&data);
    let mut dists = Vec::new();

    for state in states.nodes.iter(){
        dists.push(travel(&data, state.to_string(), true) as u64);
    }

    let mut out: u64 = 1;
    for x in dists {
        out = num::integer::lcm(out, x);
    }
    out
}

fn travel(data: &Data, pos: String, end: bool) -> u32 {

    let mut count = 0;
    let mut i = 0;
    let mut pos = pos;
    loop
    {
        let real_i = i % data.dirs.len();
        let dir = data.dirs.chars().nth(real_i).unwrap();
        let fork = data.map.get(&pos).unwrap();
        pos = match dir
        {
            'L' => fork.left.clone(),
            'R' => fork.right.clone(),
            _ => panic!("Unknown direction: {}", dir),
        };
        count += 1;
        if end {
            if pos.ends_with("Z") { break; }
        } else {
            if pos == "ZZZ" { break; }
        }
        i += 1;
    }
    count
}

fn lcm(nums: Vec<u32>) -> u64
{
    let base = &nums;
    let mut nums: Vec<u64> = nums.iter().map(|x| *x as u64).collect();
    loop {
        let min_i = min_index(&nums);
        nums[min_i] += base[min_i] as u64;
        // println!("Nums: {:?}", nums);
        if nums.iter().all(|x| *x == nums[0]) { break; }
    }
    nums[0] as u64
}

fn min_index(nums: &Vec<u64>) -> usize
{
    let mut min = nums[0];
    let mut min_i = 0;

    for (i, num) in nums.iter().enumerate()
    {
        if *num < min
        {
            min = *num;
            min_i = i;
        }
    }
    min_i
}
