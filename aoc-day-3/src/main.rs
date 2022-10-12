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

fn count_trees(map: &Vec<String>, rise: usize, run: usize) -> u64 {
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
        x = (x + run) % width;
        y += rise;
    }

    tree_count
}

fn main() {
    let map = read_file().unwrap();
    let test_slopes: Vec<(usize,usize)> = 
        vec![(1,1), (1,3), (1,5), (1,7), (2,1)];
    let tree_multi: u64 = test_slopes.iter()
        .map(|pair| count_trees(&map, pair.0, pair.1))
        .product();

    println!("The tree value is {}", tree_multi);
}
