use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;

fn read_file() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open("src/input.txt")?;
    let file = BufReader::new(file);
    let mut inputs: Vec<String> = Vec::new();

    for line in file.lines() {
        inputs.push(line?);
    }

    Ok(inputs)
}

fn main() {
    let processed = read_file().unwrap();
    for process in processed {
        println!("{}", process)
    }
}
