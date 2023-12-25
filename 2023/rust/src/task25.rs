use rand::seq::SliceRandom;
use rust::read_data;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[allow(unused)]
const TEST_DATA: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

pub fn run25_1()
{
    let data = read_data("data/data25.txt");
    // let data = TEST_DATA;
    let total = solve(&data);

    println!("Task 25_1 Solution:\nTotal: {}", total);
}

pub fn run25_2()
{
    let data = read_data("data/data25.txt");
    // let data = TEST_DATA;

    println!("There is no task 25_2");
}

fn solve(data: &str) -> u64
{
    let graph = parse_data(data);
    let mut edge_count = HashMap::new();
    const LOOPS: u64 = 1000;
    for i in 0..LOOPS {
        if i % 100 == 0 {
            println!("Loop: {}", i);
        }
        let (start, end) = get_random_se(&graph);
        let path = get_path(&graph, &start, &end);
        let edges = get_edges(&path);
        for edge in edges {
            edge_count.entry(edge).and_modify(|e| *e += 1).or_insert(1);
        }
    }
    let mut edge_counts = Vec::new();
    for (key, value) in &edge_count {
        edge_counts.push((value, key));
    }
    edge_counts.sort();
    edge_counts.reverse();
    let first_three = edge_counts[0..3].iter().map(|e| e.1).collect::<Vec<_>>();
    let (a, b) = split_set(&graph, &first_three);
    a.len() as u64 * b.len() as u64
}

fn parse_data(data: &str) -> HashMap<String, HashSet<String>>
{
    let mut graph = HashMap::new();

    for line in data.lines() {
        let mut parts = line.split(": ");
        let name = parts.next().unwrap().to_string();
        let children = parts.next().unwrap().split(" ");
        for child in children {
            let child = child.to_string();
            graph.entry(name.clone()).or_insert(HashSet::new()).insert(child.clone());
            graph.entry(child.clone()).or_insert(HashSet::new()).insert(name.clone());
        }
    }
    graph
}

fn get_path(graph: &HashMap<String, HashSet<String>>, start: &str, end: &str) -> Vec<String>
{
    let mut path = Vec::new();
    path.push(start.to_string());
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start.to_string(), path.clone()));
    while !queue.is_empty() {
        let (node, mut path) = queue.pop_front().unwrap();
        if node == end {
            return path;
        }
        visited.insert(node.clone());
        for child in graph.get(&node).unwrap() {
            if visited.contains(child) {
                continue;
            }
            let mut new_path = path.clone();
            new_path.push(child.clone());
            queue.push_back((child.clone(), new_path));
        }
    }
    Vec::new()
}

fn get_random_se(graph: &HashMap<String, HashSet<String>>) -> (String, String)
{
    let mut rng = rand::thread_rng();
    let mut keys: Vec<_> = graph.keys().collect();
    let mut start = keys.choose(&mut rng).unwrap().to_string();
    let mut end = keys.choose(&mut rng).unwrap().to_string();
    while start == end {
        end = keys.choose(&mut rng).unwrap().to_string();
    }
    (start, end)
}

fn get_edges(path: &Vec<String>) -> Vec<(String, String)>
{
    let mut edges = Vec::new();
    for i in 0..path.len() - 1 {
        let mut edge = [path[i].clone(), path[i + 1].clone()];
        edge.sort();
        edges.push((edge[0].clone(), edge[1].clone()));
    }
    edges
}

fn split_set(graph: &HashMap<String, HashSet<String>>, edges: &Vec<&(String, String)>) -> (Vec<String>, Vec<String>)
{
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(edges[0].0.clone());
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node.clone());
        a.push(node.clone());
        for child in graph.get(&node).unwrap() {
            if visited.contains(child) {
                continue;
            }
            if edges.contains(&&(node.clone(), child.clone())) || edges.contains(&&(child.clone(), node.clone())) {
                continue;
            }
                
            queue.push_back(child.clone());
        }
    }
    for (key, value) in graph {
        if !a.contains(key) {
            b.push(key.clone());
        }
    }
    (a, b)
}
