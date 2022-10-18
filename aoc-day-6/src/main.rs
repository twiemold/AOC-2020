use std::fs::{read_to_string};

fn read_file(filepath: &str) -> Result<String, std::io::Error> {
    let data = read_to_string(filepath)?;

    Ok(data)
}

fn process_data(data: String) -> Vec<String> {
    let groups: Vec<String> = data.split("\n\n").map(str::to_string).collect(); 

    groups
}

fn main() {
    let data = read_file("src/input.txt").unwrap();
    let groups = process_data(data);
    for group in groups {
        println!("Group: {}", group);
    }
}
