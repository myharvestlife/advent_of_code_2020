use std::env;
use std::fs;
use std::collections::HashMap;

mod memorizer;
use memorizer::*;

const INPUT: [u32; 6] = [11,0,1,10,5,19];
// const INPUT: [u32; 3] = [2,1,3];

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    if args.len() > 2 {
        return Err(String::from("Too many args"));
    }

    let part: u64 = args[1].to_string().parse::<u64>().unwrap();

    let contents = INPUT;

    match part {
        1 => part1(contents.to_vec()),
        2 => part2(contents.to_vec()),
        _ => return Err(String::from("Invalid Part specified")),
    }

    return Ok(());
}

fn part1(input: Vec<u32>) {
  let mut mem = Memory {
    num_map: HashMap::new(),
    curr_idx: 0,
    next_num: 0,
  };

  preload(input, &mut mem);

  let final_num = process(&mut mem, 2020);

  println!("Final # is: {}", final_num);
}

fn part2(input: Vec<u32>) {
    let mut mem = Memory {
        num_map: HashMap::new(),
        curr_idx: 0,
        next_num: 0,
    };

    preload(input, &mut mem);

    let final_num = process(&mut mem, 30000000);

    println!("Final # is: {}", final_num);
}