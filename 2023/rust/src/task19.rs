use rust::read_data;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use rand::{thread_rng, Rng, seq::SliceRandom};

#[allow(unused)]
const TEST_DATA: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

pub fn run19_1()
{
    let data = read_data("data/data19.txt");
    // let data = TEST_DATA;
    let total = solve(&data);

    println!("Task 19_1 Solution:\nTotal: {}", total);
}

pub fn run19_2()
{
    let data = read_data("data/data19.txt");
    // let data = TEST_DATA;
    let total = solve2(&data);

    println!("Task 19_2 Solution:\nTotal: {}", total);
}

fn solve(data: &str) -> u64
{
    let (items, rules) = parse(data);

    let mut total = 0;
    for item in items {
        if item.process(&rules) {
            total += item.points;
        }
    }
    total
}

fn solve2(data: &str) -> u64
{
    let (_, rules) = parse(data);
    let cur = Range::new(4000);

    let mut total = 0;
    let mut q = vec![cur];
    while q.len() > 0 {
        let cur = q.pop().unwrap();
        if cur.rule == "A" {
            total += cur.amt();
            continue;
        }
        if cur.rule == "R" {
            continue;
        }
        q.extend(cur.process(&rules));
    }
    total
}



struct Item
{
    x: u64,
    m: u64,
    a: u64,
    s: u64,
    points: u64,
}

impl Item {
    fn parse(line: &str) -> Self {

        let x = line.split("x=").last().unwrap().split(",").next().unwrap().parse::<u64>().unwrap();
        let m = line.split("m=").last().unwrap().split(",").next().unwrap().parse::<u64>().unwrap();
        let a = line.split("a=").last().unwrap().split(",").next().unwrap().parse::<u64>().unwrap();
        let s = line.split("s=").last().unwrap().split("}").next().unwrap().parse::<u64>().unwrap();

        Item {
            x,
            m,
            a,
            s,
            points: x + m + a + s,
        }
    }
    fn process(&self, rules: &HashMap<String, Vec<Rule>>) -> bool {
        let mut cur = "in".to_string();
        loop {
            if cur == "R" || cur == "A" {
                return cur == "A";
            }
            let rule = rules.get(&cur).unwrap();
            for rule in rule {
                if cur == "R" || cur == "A" {
                    return cur == "A";
                }
                if rule.category == None {
                    cur = rule.next.clone();
                    break;
                }
                match rule.category.unwrap() {
                    'x' => {
                        if rule.op.unwrap() == '<' {
                            if self.x < rule.value.unwrap() {
                                cur = rule.next.clone();
                                break;
                            }
                        } else if rule.op.unwrap() == '>' {
                            if self.x > rule.value.unwrap() {
                                cur = rule.next.clone();
                                break;
                            }
                        }
                    },
                    'm' => {
                        if rule.op.unwrap() == '<' {
                            if self.m < rule.value.unwrap() {
                                cur = rule.next.clone();
                                break;
                            }
                        } else if rule.op.unwrap() == '>' {
                            if self.m > rule.value.unwrap() {
                                cur = rule.next.clone();
                                break;
                            }
                        }
                    },
                    'a' => {
                        if rule.op.unwrap() == '<' {
                            if self.a < rule.value.unwrap() {
                                cur = rule.next.clone();
                                break;
                            }
                        } else if rule.op.unwrap() == '>' {
                            if self.a > rule.value.unwrap() {
                                cur = rule.next.clone();
                                break;
                            }
                        }
                    },
                    's' => {
                        if rule.op.unwrap() == '<' {
                            if self.s < rule.value.unwrap() {
                                cur = rule.next.clone();
                                break;
                            }
                        } else if rule.op.unwrap() == '>' {
                            if self.s > rule.value.unwrap() {
                                cur = rule.next.clone();
                                break;
                            }
                        }
                    },
                    _ => panic!("Unknown category"),
                }
            }
        }
    }
}

#[derive(Debug)]
struct Rule
{
    next: String,
    op: Option<char>,
    value: Option<u64>,
    category: Option<char>,
}

impl Rule {
    fn parse(line: &str) -> Self {
        let mut rule = Rule {
            next: "".to_string(),
            op: None,
            value: None,
            category: None,
        };
        let parts = line.split_once(":");
        match parts {
            None => {
                rule.next = line.to_string();
            }
            Some(parts) => {
                let condition = parts.0;
                rule.next = parts.1.to_string();
                rule.category = Some(condition.chars().next().unwrap());
                rule.op = Some(condition.chars().nth(1).unwrap());
                rule.value = Some(condition.split_once(rule.op.unwrap()).unwrap().1
                                  .parse::<u64>().unwrap());
            }
        }
        rule
    }
}


fn parse(data: &str) -> (Vec<Item>, HashMap<String, Vec<Rule>>)
{
    let mut items = Vec::new();
    let mut rules = HashMap::new();

    let (part1, part2) = data.split_once("\n\n").unwrap();

    for line in part1.lines() {
        let name = line.split("{").next().unwrap();
        let rules_to_parse = line.split("{").last().unwrap().split("}").next().unwrap();
        let mut rules_mem = Vec::new();
        for rule in rules_to_parse.split(",") {
            let rule = Rule::parse(rule);
            rules_mem.push(rule);
        }
        rules.insert(name.to_string(), rules_mem);
    }

    for line in part2.lines() {
        let item = Item::parse(line);
        items.push(item);
    }

    (items, rules)
}

#[derive(Debug)]
struct Range {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
    rule: String,
    pos: usize,
}

impl Range {
    fn new(range: usize) -> Self {
        Range {
            x: (1, range as u64),
            m: (1, range as u64),
            a: (1, range as u64),
            s: (1, range as u64),
            rule: "in".to_string(),
            pos: 0,
        }
    }

    fn amt(&self) -> u64 {
        // this will probably break if the non-ranges (1,0) are processed (types overflow)
        (self.x.1 - self.x.0 + 1) *
        (self.m.1 - self.m.0 + 1) *
        (self.a.1 - self.a.0 + 1) *
        (self.s.1 - self.s.0 + 1)
    }

    fn process(&self, rules: &HashMap<String, Vec<Rule>>) -> Vec<Self> {
        let mut new_ranges = Vec::new();
        let rule = &rules[&self.rule][self.pos];
        if rule.category == None {
            let new_range = Range {
                x: self.x,
                m: self.m,
                a: self.a,
                s: self.s,
                rule: rule.next.clone(),
                pos: 0,
            };
            new_ranges.push(new_range);
        } else {
            match rule.category.unwrap() {
                'x' => {
                    let (correct, wrong) = compare(self.x, rule.op.unwrap(), rule.value.unwrap());
                    new_ranges.push(Range { x: correct, m: self.m, a: self.a, s: self.s, rule: rule.next.clone(), pos: 0 });
                    new_ranges.push(Range { x: wrong, m: self.m, a: self.a, s: self.s, rule: self.rule.clone(), pos: self.pos + 1 });
                }
                'm' => {
                    let (correct, wrong) = compare(self.m, rule.op.unwrap(), rule.value.unwrap());
                    new_ranges.push(Range { x: self.x, m: correct, a: self.a, s: self.s, rule: rule.next.clone(), pos: 0 });
                    new_ranges.push(Range { x: self.x, m: wrong, a: self.a, s: self.s, rule: self.rule.clone(), pos: self.pos + 1 });
                }
                'a' => {
                    let (correct, wrong) = compare(self.a, rule.op.unwrap(), rule.value.unwrap());
                    new_ranges.push(Range { x: self.x, m: self.m, a: correct, s: self.s, rule: rule.next.clone(), pos: 0 });
                    new_ranges.push(Range { x: self.x, m: self.m, a: wrong, s: self.s, rule: self.rule.clone(), pos: self.pos + 1 });
                }
                's' => {
                    let (correct, wrong) = compare(self.s, rule.op.unwrap(), rule.value.unwrap());
                    new_ranges.push(Range { x: self.x, m: self.m, a: self.a, s: correct, rule: rule.next.clone(), pos: 0 });
                    new_ranges.push(Range { x: self.x, m: self.m, a: self.a, s: wrong, rule: self.rule.clone(), pos: self.pos + 1 });
                }
                _ => panic!("Unknown category"),
            }
        }
        new_ranges
    }
}

fn compare(range: (u64, u64), op: char, value: u64) -> ((u64, u64), (u64, u64)) {
    match op {
        // (2000, 3500) < 3000
        '<' => {
            if range.1 < value {
                // all values go through
                return (range, (1,0));
            } else if range.0 < value {
                return ((range.0, value - 1), (value, range.1));
            } else {
                return ((1,0), range);
            }
        }
        '>' => {
            if range.0 > value {
                // all values go through
                return (range, (1,0));
            } else if range.1 > value {
                return ((value + 1, range.1), (range.0, value));
            } else {
                return ((1,0), range);
            }
        }
        _ => panic!("Unknown op"),
    }
}
