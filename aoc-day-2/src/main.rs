use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn read_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("src/input.txt")?;
    let file = BufReader::new(file);

    for line in file.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn main() {
    let num = read_file();
}
