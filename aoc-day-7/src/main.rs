use std::fs::{read_to_string};

fn read_file(filepath: &str) -> String {
    let data = read_to_string(filepath).unwrap();

    data
}

fn main() {
    let data = read_file("src/input.txt");
    println!("{}", data);
}
