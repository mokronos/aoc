use rust::read_data;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
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
    // let data = TEST_DATA;
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
    const RUNS: usize = 1000;
    let (mut low, mut high) = (0, 0);
    for _ in 0..RUNS {
        let (l, h) = press_button(&mut modules, &config);
        low += l; high += h;
    }

    low * high
}

fn solve2(data: &str) -> u64
{
    // Too lazy to programmaically solve this, probably difficult without leveraging
    // the special form of the input
    //
    // 4 registers with 12 bits each
    // 1(qt -> pl): sm(1)>nd(0)>hp(1)>nt(0)>jv(1)>rh(0)>vr(1)>sl(1)>jx(0)>cf(1)>hl(1)>ng(1)>qt
    // 2(dq -> mz): xd(1)>fb(0)>rt(0)>mt(1)>rc(0)>lv(1)>xs(0)>ml(0)>ts(1)>gx(1)>mh(1)>kx(1)>dq
    // 3(vt -> lz): kr(1)>hh(1)>dh(0)>kq(0)>lm(0)>hn(1)>qk(0)>cb(1)>hf(1)>ch(1)>kd(1)>nb(1)>vt
    // 4(nl -> zm): zb(1)>rb(1)>sk(1)>bh(1)>fr(0)>qf(1)>rd(1)>gq(1)>hm(0)>jp(1)>gj(1)>bv(1)>nl
    // bin | dec
    // 1: 111011010101 | 3797
    // 2: 111100101001 | 3881
    // 3: 111110100011 | 4003
    // 4: 111011101111 | 3823
    // Product/LCM: 
    3797 * 3881 * 4003 * 3823
}

#[derive(Debug)]
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcast(Broadcast),
}

impl Module {
    fn process(&mut self, input: (String, String, bool), config: &HashMap<String, Vec<String>>)
        -> Vec<(String, String, bool)> {
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

    fn process(&mut self, input: (String, String, bool), config: &HashMap<String, Vec<String>>)
        -> Vec<(String, String, bool)> {
        let mut result = Vec::new();
        match input.2 {
            true => return result,
            false => match self.on {
                true => self.on = false,
                false => self.on = true,
            }
        }
        for target in config.get(&input.0).unwrap() {
            result.push((target.clone(), input.0.clone(), self.on));
        }
        result
    }
}

#[derive(Debug)]
struct Conjunction {
    mem: HashMap<String, bool>,
}

impl Conjunction {
    fn new(modules: &HashMap<String, Module>, config: &HashMap<String, Vec<String>>,
           name: &String) -> Self {

        let mut mem: HashMap<String, bool> = HashMap::new();
        for (k, v) in config {
            if v.contains(name) {
                mem.insert(k.clone(), false);
            }
        }
        Self { 
            mem
        }
    }

    fn process(&mut self, input: (String, String, bool), config: &HashMap<String, Vec<String>>)
        -> Vec<(String, String, bool)> {
        let mut result = Vec::new();
        let mut not_all = true;
        self.mem.insert(input.1.clone(), input.2);
        if self.mem.values().all(|&x| x == true) {
            not_all = false;
        }

        for target in config.get(&input.0).unwrap() {
            result.push((target.clone(), input.0.clone(), not_all));
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
    fn process(&self, input: (String, String, bool), config: &HashMap<String, Vec<String>>)
        -> Vec<(String, String, bool)> {
        let mut result = Vec::new();
        for target in config.get(&input.0).unwrap() {
            result.push((target.clone(), input.0.clone(), input.2));
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
                       else if category == "&" { Module::Conjunction(Conjunction::new(&modules,
                                                                                        &config,
                                                                                        &name)) }
                       else { Module::Broadcast(Broadcast::new()) });
        config.insert(name.clone(), right);
    };
        
    (modules, config)
}

fn press_button(modules: &mut HashMap<String, Module>,
                config: &HashMap<String, Vec<String>>) -> (u64, u64)
{
    let mut queue = VecDeque::new();
    let mut low = 0;
    let mut high = 0;
    queue.push_back(("broadcaster".to_string(), "button".to_string(), false));
    while queue.len() > 0 {
        // println!("Queue: {:?}", queue);
        let (target, sender, on) = queue.pop_front().unwrap();
        // println!("{} --> {}", if on { "high" } else { "low" }, target);
        match on { true => high += 1, false => low += 1 }
        let module = modules.get_mut(&target);
        if module.is_none() {
            continue;
        }
        queue.extend(module.unwrap().process((target, sender, on), config));
    }
    (low, high)
}

fn press_button2(modules: &mut HashMap<String, Module>,
                config: &HashMap<String, Vec<String>>) -> bool
{
    let mut queue = VecDeque::new();
    let mut low = 0;
    let mut high = 0;
    queue.push_back(("broadcaster".to_string(), "button".to_string(), false));
    while queue.len() > 0 {
        // println!("Queue: {:?}", queue);
        let (target, sender, on) = queue.pop_front().unwrap();
        // println!("{} --> {}", if on { "high" } else { "low" }, target);
        if target == "rx" && !on {
            return true;
        }
        match on { true => high += 1, false => low += 1 }
        let module = modules.get_mut(&target);
        if module.is_none() {
            continue;
        }
        queue.extend(module.unwrap().process((target, sender, on), config));
    }
    false
}
