use std::env;
use std::fs;
use std::collections::HashMap;

use utils::input_converter;

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
    let mut data = input_converter::to_vec32(input);

    data.sort();

    let mut ones = 0;
    let mut threes = 0;

    if data[0] == 3 {
        threes += 1;
    } else if data[0] == 1 {
        ones += 1;
    }

    for n in 0..data.len() {
        if n < data.len() - 1 {
            if data[n + 1] - data[n] == 3 {
                threes += 1;
            } else if data[n + 1] - data[n] == 1 {
                ones += 1;
            } else {
                println!("Could not locate a proper difference between {} and {}", data[n], data[n + 1]);
            }
        }
    }

    threes += 1;

    println!("Found {} Ones and {} Threes -- Product: {}", ones, threes, ones * threes);
}

fn part2(input: String) {
    let mut data = input_converter::to_vec64(input);

    data.sort();

    let num_configurations = get_num_configurations(&data);

    println!("Number of configurations: {}", num_configurations);
}

fn get_num_configurations(data: &Vec<u64>) -> u64 {
    let mut num_configurations = 0;
    let mut already_processed: HashMap<usize, u64> = HashMap::new();

    if data[0] <= 3 {
        num_configurations += process_chain(&data, 0, &mut already_processed);
    }

    if data[1] <= 3 {
        num_configurations += process_chain(&data, 1, &mut already_processed);
    }

    if data[2] <= 3 {
        num_configurations += process_chain(&data, 2, &mut already_processed);
    }

    return num_configurations;
}

fn process_chain(data: &Vec<u64>, idx: usize, already_processed: &mut HashMap<usize, u64>) -> u64 {
    // println!("Checking IDX: {}", idx);
    if already_processed.contains_key(&idx) {
        println!("Already Processed {}", idx);
        return *already_processed.get(&idx).unwrap();
    }

    let mut sum = 0;

    // If we hit the last value, return 1 to start collapsing values
    if idx == data.len() - 1 {
        println!("Hit a last index!");
        return 1;
    }

    if idx + 1 < data.len() && data[idx + 1] - data[idx] <= 3 {
        sum += process_chain(data, idx + 1, already_processed);
    }

    if idx + 2 < data.len() && data[idx + 2] - data[idx] <= 3 {
        sum += process_chain(data, idx + 2, already_processed);
    }

    if idx + 3 < data.len() && data[idx + 3] - data[idx] <= 3 {
        sum += process_chain(data, idx + 3, already_processed);
    }

    already_processed.insert(idx, sum);
    return sum;
}
