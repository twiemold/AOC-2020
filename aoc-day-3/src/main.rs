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

fn count_trees(map: Vec<String>) -> u32 {
    let mut x = 0;
    let mut y = 0;
    let mut tree_count = 0;
    let width = map[0].len();
    let length = map.len();
    while y < length {
        let mark = map[y].chars().nth(x).unwrap();
        if mark == '#' {
            tree_count += 1;
        }
        x = (x + 3) % width;
        y += 1;
    }

    tree_count
}

fn main() {
    let map = read_file().unwrap();
    let tree_count = count_trees(map);

    println!("The tree count is {}", tree_count);
}
