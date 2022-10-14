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

fn process_tickets(tickets: Vec<String>, row_count: u8, column_count: u8) -> u32 {
    let mut max_id: u32 = 0;
    let row_vec = Vec::from_iter(0..row_count);
    let col_vec = Vec::from_iter(0..column_count);
    for ticket in tickets {
        let mut l = 0;
        let mut r = row_vec.len();
        let mut mid = 0;
        for i in 0..7 {
            mid = (l + r) / 2;
            let letter = ticket.chars().nth(i).unwrap();
            match letter {
                'F' => r = mid,
                'B' => l = mid,
                _ => (),
            }
        }
        let row: u32 = row_vec[mid].into();
        l = 0;
        r = col_vec.len();
        mid = 0;
        for i in 7..10 {
            mid = (l + r) / 2;
            let letter = ticket.chars().nth(i).unwrap();
            match letter {
                'L' => r = mid,
                'R' => l = mid,
                _ => (),
            }
        }
        let column: u32 = col_vec[mid].into();
        if row*8+column > max_id {
            max_id = row*8+column
        }

    }

    max_id
}

fn main() {
   let tickets = read_file("src/input.txt").unwrap();
   let max_id = process_tickets(tickets, 127, 7);
   println!("The max value is {}", max_id);
}
