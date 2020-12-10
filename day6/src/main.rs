use std::env;
use std::fs;
use regex::Regex;
use std::collections::HashMap;

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

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn part1(input: String) {
    let groups: Vec<&str> = input.split("\n\n").collect();

    let mut total: u32 = 0;
    for group in groups {
        let group_sum = sum_group(group);
        total += group_sum;
    }

    println!("Total: {}", total);
}

fn sum_group(group: &str) -> u32 {
    let mut total: u32 = 0;
    let group_string = group.to_string();

    for letter in ALPHABET.chars() {
        if group_string.contains(letter) {
            total += 1;
        }
    }

    return total;
}

fn part2(input: String) {
    let groups: Vec<&str> = input.split("\n\n").collect();

    let mut total: u32 = 0;
    for group in groups {
        let group_sum = sum_group_all(group);
        total += group_sum;
    }

    println!("Total: {}", total);
}

fn sum_group_all(group: &str) -> u32 {
    let mut total: u32 = 0;
    let group_string = group.to_string();
    let mut map = HashMap::new();

    let lines: Vec<&str> = group_string.split("\n").collect();

    println!("Lines -- {:?}", lines.clone());

    let mut num_lines = 0;

    for line in lines {
        if line.len() == 0 {
            continue;
        }

        num_lines += 1;

        for letter in line.chars() {
            let count = map.entry(letter.to_string()).or_insert(0);
            *count += 1;
        }
    }

    println!("DEBUG -- num_lines: {}, map: {:?}", num_lines, map);

    for key in map.keys() {
        let num = match map.get(key) {
            Some(i) => *i,
            None => 0,
        };


        if num == num_lines {
            total += 1;
        }
    }

    println!("TOTAL {}", total);

    return total;
}

