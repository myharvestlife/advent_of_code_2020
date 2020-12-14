use std::env;
use std::fs;
use utils::input_converter;

mod mask;
use mask::*;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    if args.len() > 2 {
        return Err(String::from("Too many args"));
    }

    let part: u64 = args[1].to_string().parse::<u64>().unwrap();

    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    match part {
        1 => part1(contents),
        2 => part2(contents),
        _ => return Err(String::from("Invalid Part specified")),
    }

    return Ok(());
}

fn part1(input: String) {
    let lines = input_converter::to_string_vec(input);

    let mut mem = Memory {
        mask: ['0'; 36].to_vec(),
        memory: [0; 36],
    };

    for line in lines {
        if parser::is_mask(line) {
            let mask = parser::get_mask(line);

            // make it start from the 0 index on the left by reversing
            mem.mask = mask.chars().rev().collect();
        } else {
            let (mem_loc, mem_val) = parser::get_mem(line);


        }
    }
}

fn part2(input: String) {}