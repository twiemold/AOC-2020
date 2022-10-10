use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use std::fmt;

pub struct PasswordInput {
    min: u8,
    max: u8,
    letter: char,
    password: String,
}

impl fmt::Display for PasswordInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "min: {} max: {} char: {} password: {}", self.min, self.max, self.letter, self.password)
    }
}

fn read_file() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open("src/input.txt")?;
    let file = BufReader::new(file);
    let mut inputs: Vec<String> = Vec::new();

    for line in file.lines() {
        inputs.push(line?);
    }

    Ok(inputs)
}

fn process_inputs(inputs: Vec<String>) -> Result<Vec<PasswordInput>, Box<dyn std::error::Error>> {
    let re = Regex::new(
        r"(?P<num1>^\d{1,2})[-](?P<num2>\d{1,2})\s(?P<letter>[a-z])[:]\s(?P<pass>\w+)").unwrap();
    let mut proccessed: Vec<PasswordInput> = Vec::new();
    for input in inputs {
        let caps = re.captures(input.as_str()).unwrap();
        let new_pass = PasswordInput {
            min: caps.name("num1").unwrap().as_str().parse().unwrap(),
            max: caps.name("num2").unwrap().as_str().parse().unwrap(),
            letter: caps.name("letter").unwrap().as_str().chars().nth(0).unwrap(),
            password: caps.name("pass").unwrap().as_str().to_string(),
        };
        proccessed.push(new_pass);
    }

    Ok(proccessed)
}

fn main() {
    let inputs = read_file();
    let proccessed = process_inputs(inputs.unwrap()).unwrap();
    for item in proccessed {
        println!("{}", item);
    }

}
