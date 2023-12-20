use rust::read_data;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use rand::{thread_rng, Rng, seq::SliceRandom};

#[allow(unused)]
const TEST_DATA: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

pub fn run20_1()
{
    let data = read_data("data/data20.txt");
    let data = TEST_DATA;
    let total = solve(&data);

    println!("Task 20_1 Solution:\nTotal: {}", total);
}

pub fn run20_2()
{
    let data = read_data("data/data20.txt");
    // let data = TEST_DATA;
    let total = solve2(&data);

    println!("Task 20_2 Solution:\nTotal: {}", total);
}

fn solve(data: &str) -> u64
{
    let (mut modules, config) = parse(&data);
    let (low, high) = press_button(&mut modules, &config);
    println!("Low: {}, High: {}", low, high);
    0
}

fn solve2(data: &str) -> u64
{

    0
}

#[derive(Debug)]
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcast(Broadcast),
}

impl Module {
    fn process(&mut self, input: (String, bool), config: &HashMap<String, Vec<String>>) -> Vec<(String, bool)> {
        match self {
            Module::FlipFlop(f) => f.process(input, config),
            Module::Conjunction(c) => c.process(input, config),
            Module::Broadcast(b) => b.process(input, config),
        }
    }
}

#[derive(Debug)]
struct FlipFlop {
    on: bool,
}

impl FlipFlop {
    fn new() -> Self {
        Self { 
            on: false,
        }
    }

    fn process(&mut self, input: (String, bool), config: &HashMap<String, Vec<String>>) -> Vec<(String, bool)> {
        let mut result = Vec::new();
        match input.1 {
            true => return result,
            false => match self.on {
                true => self.on = false,
                false => self.on = true,
            }
        }
        for target in config.get(&input.0).unwrap() {
            result.push((target.clone(), self.on));
        }
        result
    }
}

#[derive(Debug)]
struct Conjunction {
    mem: HashMap<String, bool>,
}

impl Conjunction {
    fn new(modules: &HashMap<String, Module>) -> Self {
        let mem: HashMap<String, bool> = modules.keys().map(|k| (k.clone(), false)).collect();
        Self { 
            mem
        }
    }

    fn process(&mut self, input: (String, bool), config: &HashMap<String, Vec<String>>) -> Vec<(String, bool)> {
        let mut result = Vec::new();
        let mut not_all = true;
        self.mem.insert(input.0.clone(), input.1);
        if self.mem.values().all(|&x| x == true) {
            not_all = false;
        }

        for target in config.get(&input.0).unwrap() {
            result.push((target.clone(), not_all));
        }
        result
    }
}

#[derive(Debug)]
struct Broadcast {
}

impl Broadcast {
    fn new() -> Self {
        Self { }
    }
    fn process(&self, input: (String, bool), config: &HashMap<String, Vec<String>>) -> Vec<(String, bool)> {
        let mut result = Vec::new();
        for target in config.get(&input.0).unwrap() {
            result.push((target.clone(), input.1));
        }
        result
    }
}

fn parse(data: &str) -> (HashMap<String, Module>, HashMap<String, Vec<String>>)
{
    let mut modules = HashMap::new();
    let mut config = HashMap::new();

    for line in data.lines() {
        let mut parts = line.split(" -> ");
        let mut left = parts.next().unwrap();
        let category = left[..1].to_string();
        let name = match category.as_str() {
            "%" | "&" => left[1..].to_string(),
            _ => left.to_string(),
        };
        let mut right = parts.next().unwrap().split(", ").map(|s| s.to_string()).collect::<Vec<String>>();
        modules.insert(name.clone(), if category == "%" { Module::FlipFlop(FlipFlop::new()) }
                       else if category == "&" { Module::Conjunction(Conjunction::new(&modules)) }
                       else { Module::Broadcast(Broadcast::new()) });
        config.insert(name.clone(), right);
    };
        
    (modules, config)
}

fn press_button(modules: &mut HashMap<String, Module>,
                config: &HashMap<String, Vec<String>>) -> (u64, u64)
{
    let mut queue = BinaryHeap::new();
    let mut low = 0;
    let mut high = 0;
    queue.push(("broadcaster".to_string(), false));
    while queue.len() > 0 {
        let (name, on) = queue.pop().unwrap();
        println!("{} --> {}", if on { "high" } else { "low" }, name);
        match on { true => high += 1, false => low += 1 }
        let module = modules.get_mut(&name);
        if module.is_none() {
            continue;
        }
        queue.extend(module.unwrap().process((name.clone(), on), &config));
    }
    (low, high)
}
    
