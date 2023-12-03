use rust::read_data;
use std::collections::HashMap;

pub fn run1_1()
{
    let data = read_data("data/data1.txt");

    let mut total: u32 = 0;
    for line in &data {
        let mut first = 'x';
        let mut last = 'x';
        for c in line.chars() {
            if c.is_ascii_digit() {
                if first == 'x' {
                    first = c;
                }
                last = c;
            }
        }
        let first = first.to_digit(10).unwrap();
        let last = last.to_digit(10).unwrap();
        let num = first * 10 + last;
        total += num;
    }
    println!("Task 1_1 Solution:\nTotal: {}", total);
}

pub fn run1_2()
{
    let data = read_data("data/data1.txt");

    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("one".to_string(), 1);
    map.insert("two".to_string(), 2);
    map.insert("three".to_string(), 3);
    map.insert("four".to_string(), 4);
    map.insert("five".to_string(), 5);
    map.insert("six".to_string(), 6);
    map.insert("seven".to_string(), 7);
    map.insert("eight".to_string(), 8);
    map.insert("nine".to_string(), 9);
    for i in 0..10 {
        map.insert(i.to_string(), i);
    }

    let mut total: u32 = 0;
    for line in &data {
        let mut first = 10;
        let mut last = 10;
        for i in 0..line.len() {
            let start = &line[i..];
            for (key, value) in &map {
                if start.starts_with(key) {
                    if first == 10 {
                        first = *value
                    }
                    last = *value
                }
            }
        }
        let first = first;
        let last = last;
        let num = first * 10 + last;
        total += num;
    }
    println!("Task 1_2 Solution:\nTotal: {}", total);
}
