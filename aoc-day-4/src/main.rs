use std::fs::{read_to_string};
use regex::Regex;

pub struct Passport {
    byr: Option<u16>,
    iyr: Option<u16>,
    eyr: Option<u16>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<u16>,
}

impl Default for Passport {
    fn default() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None, 
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }
}

fn read_file(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = read_to_string(filepath)?;

    Ok(data)
}

fn process_input(input: String) -> Result<Vec<Passport>, Box<dyn std::error::Error>> {
    let passport: Vec<&str> = input.split("\n\n").collect();
    let mut passport_vec: Vec<Passport> = Vec::new();
    for data in passport {
        let mut passport_vals: Vec<(&str, &str)> = Vec::new();
        let mut temp_passport = Passport { ..Default::default() };
        for line in data.split_whitespace() {        
            let field = &line[0..3];
            let data = &line[4..];
            passport_vals.push((field, data));
        }
        for pair in passport_vals {
            match pair.0 {
                "byr" => {
                    let data = pair.1.parse().unwrap();
                    if data >= 1920 && data <= 2002 {
                        temp_passport.byr = Some(data);
                    }
                },
                "iyr" => {
                    let data = pair.1.parse().unwrap();
                    if data >= 2010 && data <= 2020 {
                        temp_passport.iyr = Some(data);
                    }
                },
                "eyr" => {
                    let data = pair.1.parse().unwrap();
                    if data >= 2020 && data <= 2030 {
                        temp_passport.eyr = Some(data);
                    }
                },
                "hgt" => {
                    let len = pair.1.len();
                    let unit = &pair.1[(len-1)-1..];
                    if unit == "cm" {
                        let data: u16 = pair.1[..(len-1)-1].parse().unwrap();
                        if data >= 150 && data <= 193 {
                            temp_passport.hgt = Some(pair.1.to_string());
                        }
                    } else if unit == "in" {
                        let data: u16 = pair.1[..(len-1)-1].parse().unwrap();
                        if data >= 59 && data <= 76 {
                            temp_passport.hgt = Some(pair.1.to_string());
                        }
                    }
                }, 
                "hcl" => {
                    let re = Regex::new(r"#[a-z,0-9]{6}").unwrap();
                    let cap = re.captures(pair.1);
                    match cap {
                        Some(val) => temp_passport.hcl = Some(val.get(0).unwrap().as_str().to_string()),
                        None => (),
                    }
                },
                "ecl" => {
                    let data = pair.1;
                    match data {
                        "amb" => temp_passport.ecl = Some(pair.1.to_string()),
                        "blu" => temp_passport.ecl = Some(pair.1.to_string()),
                        "brn" => temp_passport.ecl = Some(pair.1.to_string()),
                        "gry" => temp_passport.ecl = Some(pair.1.to_string()),
                        "grn" => temp_passport.ecl = Some(pair.1.to_string()),
                        "hzl" => temp_passport.ecl = Some(pair.1.to_string()),
                        "oth" => temp_passport.ecl = Some(pair.1.to_string()),
                        _ => (),
                    }
                },
                "pid" => {
                    let re = Regex::new(r"^[0-9]{9}$").unwrap();
                    let cap = re.captures(pair.1);
                    match cap {
                        Some(val) => temp_passport.pid = Some(val.get(0).unwrap().as_str().to_string()),
                        None => (),
                    }
                },

                "cid" => temp_passport.cid = Some(pair.1.parse().unwrap()),
                _other => panic!("Bad data"),
            };
        }
        passport_vec.push(temp_passport);

    }

    Ok(passport_vec)
}

fn count_valid(passports: Vec<Passport>) -> u32 {
    let mut count = 0;

    for passport in passports {
        if passport.byr.is_some() &&
            passport.iyr.is_some() &&
            passport.eyr.is_some() &&
            passport.hgt.is_some() &&
            passport.hcl.is_some() &&
            passport.ecl.is_some() &&
            passport.pid.is_some() {
                count += 1
        }
    }
    count
}

fn main() {
    let data = read_file("src/input.txt").unwrap();
    let passports = process_input(data).unwrap();
    let valid = count_valid(passports);

    println!("The number of valid passports is {}", valid);
}
