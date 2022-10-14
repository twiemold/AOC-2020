use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;

fn read_file(filepath: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let file = BufReader::new(file);
    let mut inputs: Vec<String> = Vec::new();

    for line in file.lines() {
        inputs.push(line?);
    }

    Ok(inputs)
}

fn main() {
   let inputs = read_file("src/input.txt").unwrap();
}
