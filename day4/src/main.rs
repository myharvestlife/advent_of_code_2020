use std::env;
use std::fs;
use std::str;
use regex::Regex;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    if args.len() > 2 {
        return Err(String::from("Too many args"));
    }

    let part: u32 = args[1].to_string().parse::<u32>().unwrap();

    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    match part {
        1 => part1(contents),
        2 => part2(contents),
        _ => return Err(String::from("Invalid Part specified")),
    }

    return Ok(());
}

fn part1(input: String) {
    let required_fields: Vec<&str> = [
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
    ].to_vec();

    let optional_fields: Vec<&str> = [
        "cid",
    ].to_vec();

    let rows = input.split("\n\n");

    let mut num_valid = 0;

    for row in rows {
        println!("Passport Row: {}", row);
        let fields = row.split(&[' ', '\n'][..]);

        if has_all_required(required_fields.clone(), fields.collect()) {
            num_valid += 1;
        }
    }

    println!("Number of Valid Passports: {}", num_valid);
}

fn has_all_required(required: Vec<&str>, fields: Vec<&str>) -> bool {
    for req in required {
        let mut found = false;

        for field in fields.clone() {
            if field == "" {
                continue;
            }

            let split_field: Vec<&str> = field.split(':').collect();
            let first = split_field[0];

            if first == req {
                println!("Found {} matching {}", first, req);
                found = true;
                break;
            }
        }

        if !found {
            println!("Could not find all required fields in {:?}", fields);
            return false;
        }
    }

    println!("PASSPORT VALID");
    return true;
}

fn part2(input: String) {
    let rows = input.split("\n\n");

    let mut num_valid = 0;

    for row in rows {
        println!("Passport Row: {}", row);
        let fields: Vec<&str> = row.split(&[' ', '\n'][..]).collect();

        if is_valid(fields.clone()) {
            num_valid += 1;
            println!("Row is valid {:?}", fields);
        }
    }

    println!("Number of Valid Passports: {}", num_valid);
}

fn is_valid(fields: Vec<&str>) -> bool {
    let mut validations: [bool; 7] = [false; 7];
    let hair_regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    let eye_regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    let pid_regex = Regex::new(r"^\d{9}$").unwrap();

    for field in fields.clone() {
        if field == "" {
            continue;
        }

        let split_field: Vec<&str> = field.split(':').collect();
        let first = split_field[0];
        let second = split_field[1];

        if first == "byr" {
            let year = match second.parse::<i32>() {
                Ok(value) => value,
                Err(_err) => {
                    println!("Could not parse {} as int!", second);
                    return false;
                }
            };

            if year > 2002 || year < 1920 {
                return false;
            }

            validations[0] = true;
        }

        if first == "iyr" {
            let year = match second.parse::<i32>() {
                Ok(value) => value,
                Err(_err) => {
                    println!("Could not parse {} as int!", second);
                    return false;
                }
            };

            if year > 2020 || year < 2010 {
                return false;
            }

            validations[1] = true;
        }

        if first == "eyr" {
            let year = match second.parse::<i32>() {
                Ok(value) => value,
                Err(_err) => {
                    println!("Could not parse {} as int!", second);
                    return false;
                }
            };

            if year > 2030 || year < 2020 {
                return false;
            }

            validations[2] = true;
        }

        if first == "hgt" {
            let height_val: (i32, String) = get_height(second);
            if height_val.1 == "in" {
                if height_val.0 > 76 || height_val.0 < 59 {
                    return false;
                } else {
                    validations[3] = true;
                }
            } else if height_val.1 == "cm" {
                if height_val.0 > 193 || height_val.0 < 150 {
                    return false;
                } else {
                    validations[3] = true;
                }
            } else {
                return false;
            }
        }

        if first == "hcl" {
            if hair_regex.is_match(second) {
                validations[4] = true;
            } else {
                return false;
            }
        }

        if first == "ecl" {
            if eye_regex.is_match(second) {
                validations[5] = true;
            } else {
                return false;
            }
        }

        if first == "pid" {
            if pid_regex.is_match(second) {
                validations[6] = true;
            } else {
                return false;
            }
        }
    }

    for validation in validations.iter() {
        if *validation == false {
            return false;
        }
    }

    return true;
}

fn get_height(height: &str) -> (i32, String) {
    let re = Regex::new(r"^(?P<height>\d+)(?P<height_type>in|cm)$").unwrap();

    let captures = match re.captures(height) {
        Some(caps) => caps,
        None => return (0, String::from("")),
    };

    let height = captures["height"].parse::<i32>().expect("Failed to unwrap height");
    let height_type = captures["height_type"].to_string();

    return (height, height_type);
}