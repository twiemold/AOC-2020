use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;

fn read_file(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut data = Vec::new();

    for line in reader.lines() {
        data.push(line.unwrap());
    }

    data
}

fn run_program(program: Vec<String>) -> isize {
    let mut acc: isize = 0;
    let mut line_set = HashSet::new();
    let mut line_num: isize = 0;
    let mut not_repeat: bool = true;
    line_set.insert(line_num);
    while not_repeat {
        let instr = &program[line_num as usize];
        let instr_type = &instr[..3];
        let instr_val_sign = &instr[4..5];
        let instr_val: isize = instr[5..].parse().unwrap();
        // println!("Sign: {}, Val: {}", instr_val_sign, instr_val);
        match instr_type {
            "nop" => line_num += 1,
            "jmp" => { 
                if instr_val_sign == "+" {line_num += instr_val;} else {line_num -= instr_val;}
            },
            "acc" => {
                if instr_val_sign == "+" {acc += instr_val;} else {acc -= instr_val;}
                line_num += 1;
            },
            _ => panic!("Bad data")
        }
        not_repeat = line_set.insert(line_num);
    }

    acc
}

fn main() {
    let data = read_file("src/input.txt");
    let acc = run_program(data);
    println!("Accumulator value is: {acc}");
}
