use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

fn read_file(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut data = Vec::new();

    for line in reader.lines() {
        data.push(line.unwrap());
    }

    data
}

fn main() {
    let data = read_file("src/input.txt");
    for line in data {
        println!("{line}");
    }
}
