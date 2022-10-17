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

fn process_tickets(tickets: Vec<String>, row_count: u8, column_count: u8) -> (Vec<u32>, u32) {
    let mut max_id: u32 = 0;
    let mut seat_ids: Vec<u32> = Vec::new();
    let row_vec = Vec::from_iter(0..row_count);
    let col_vec = Vec::from_iter(0..column_count);
    for ticket in tickets {
        let mut l = 0;
        let mut r = row_vec.len()-1;
        let mut mid = 0;
        for i in 0..6 {
            mid = l + ((r-l) / 2);
            let letter = ticket.chars().nth(i).unwrap();
            match letter {
                'F' => r = mid,
                'B' => l = mid+1,
                _ => (),
            }
        }
        if ticket.chars().nth(6).unwrap() == 'F' {
            mid = l;
        } else if ticket.chars().nth(6).unwrap() == 'B' {
            mid = r;
        }
        let row: u32 = row_vec[mid].into();
        l = 0;
        r = col_vec.len()-1;
        for i in 7..9 {
            mid = l + ((r-l) / 2);
            let letter = ticket.chars().nth(i).unwrap();
            match letter {
                'L' => r = mid,
                'R' => l = mid+1,
                _ => (),
            }
        }
        if ticket.chars().nth(9).unwrap() == 'L' {
            mid = l;
        } else if ticket.chars().nth(9).unwrap() == 'R' {
            mid = r;
        }
        let column: u32 = col_vec[mid].into();
        // println!("Column: {}", column);
        let seat_id = row*8+column; 
        if seat_id  > max_id {
            max_id = seat_id;
        }
        seat_ids.push(seat_id);

    }

    (seat_ids, max_id)
}

fn find_missing(mut seat_ids: Vec<u32>) -> u32 {
    seat_ids.sort();
    let mut missing = 0;
    let mut i = 0;
    while i < seat_ids.len()-1 && seat_ids[i]+1 == seat_ids[i+1] {
        i += 1
    }
    missing = seat_ids[i]+1;
    missing
}

fn main() {
   let tickets = read_file("src/input.txt").unwrap();
   let seat_vals = process_tickets(tickets, 128, 8);
   let seat_ids = seat_vals.0;
   let max_id = seat_vals.1;
   let missing = find_missing(seat_ids);
   println!("Your ticket id is #{}", missing);
}
