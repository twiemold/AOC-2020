use std::fs::{read_to_string};

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
            // let caps = re.captures(line).unwrap();
            // let field = caps.name("field").unwrap().as_str();
            // let value = caps.name("value").unwrap().as_str();
            // println!("Field: {}, Value: {}", field, value);
        }
        for pair in passport_vals {
            match pair.0 {
                "byr" => temp_passport.byr = Some(pair.1.parse().unwrap()),
                "iyr" => temp_passport.iyr = Some(pair.1.parse().unwrap()),
                "eyr" => temp_passport.eyr = Some(pair.1.parse().unwrap()),
                "hgt" => temp_passport.hgt = Some(pair.1.to_string()),
                "hcl" => temp_passport.hcl = Some(pair.1.to_string()),
                "ecl" => temp_passport.ecl = Some(pair.1.to_string()),
                "pid" => temp_passport.pid = Some(pair.1.to_string()),
                "cid" => temp_passport.cid = Some(pair.1.parse().unwrap()),
                _other => panic!("Bad data"),
            };
        }
        passport_vec.push(temp_passport);

    }

    Ok(passport_vec)
}



fn main() {
    let data = read_file("src/input.txt").unwrap();
    let passports = process_input(data).unwrap();
}
