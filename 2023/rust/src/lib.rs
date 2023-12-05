use std::fs;

pub fn read_data(path: &str) -> String
{
    fs::read_to_string(path).expect("Something went wrong reading the file")
}
