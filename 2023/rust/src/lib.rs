use std::io::{BufReader, BufRead};
use::std::fs::File;

pub fn read_data(path: &str) -> Vec<String>
{
    let mut data = Vec::new();
    let file = File::open(path).expect("file not found");
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        data.push(line.expect("Could not parse line"));
    }
    data
}
