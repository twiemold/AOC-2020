use std::fs::{read_to_string};
use std::collections::HashMap;

fn read_file(filepath: &str) -> Result<String, std::io::Error> {
    let data = read_to_string(filepath)?;

    Ok(data)
}

fn process_data(data: String) -> Vec<String> {
    let groups: Vec<String> = data.split("\n\n").map(str::to_string).collect(); 

    groups
}

fn sum_questions(groups: Vec<String>) -> u32 {
    let mut key_count = 1;
    for group in groups {
        let head_count = group.chars().filter(|letter| *letter == '\n').count() + 1;
        let mut sub_count: u32 = 0;
        let mut group_hash = HashMap::new();
        let _val = group
            .chars()
            .filter(|letter| *letter != '\n')
            .map(|letter| *group_hash.entry(letter).or_insert(0) += 1)
            .count();
        sub_count += group_hash.keys().filter(|key| group_hash[key] == head_count).count() as u32;
        key_count += sub_count;
    }

    key_count
}

fn main() {
    let data = read_file("src/input.txt").unwrap();
    let groups = process_data(data);
    let sum = sum_questions(groups);
    println!("The sum of the counts is {}", sum);
}
