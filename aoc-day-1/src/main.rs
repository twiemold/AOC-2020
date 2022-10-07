use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn read_file() -> Result<Vec<u16>, io::Error> {
    let file = File::open("src/input.txt")?;
    let file = BufReader::new(file);
    let mut nums: Vec<u16> = Vec::new();

    for line in file.lines() {
        let new_int = line.unwrap().parse::<u16>().unwrap();
        nums.push(new_int);
    }
    // for num in &nums {
    //     println!("{}", num);
    // }

    Ok(nums)
}

// fn find_sum(nums: vec<u16>) -> (u16, u16) {
//     let mut nums_hash = hashmap::new();
// 
//     
// }

fn main() {
    let nums = read_file();
}
