use std::env;
use std::fs;
use math::round;
use num::integer::lcm;

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

fn parse_input(input: String) -> (u32, Vec<u32>) {
    let split: Vec<&str> = input.split("\n").collect();
    let start_time: u32 = split[0].parse().unwrap();
    let busses: Vec<u32> = split[1].split(",").filter(|n| *n != "x").map(|n| n.parse().unwrap()).collect();
    return (start_time, busses);
}

fn part1(input: String) {
    let (start_time, busses) = parse_input(input);

    println!("start_time: {} -- busses: {:?}", start_time, busses);
    let mut closest_bus: u32 = 0;
    let mut closest_diff: u32 = 100000000;

    for i in 0..busses.len() {
        let lower = (start_time / busses[i]) * busses[i];
        let upper = lower + busses[i];

        // if it's exactly equal, this is the closest time
        if lower == start_time {
            closest_bus = busses[i];
            closest_diff = 0;
            break;
        }

        let diff = upper - start_time;

        if diff < closest_diff {
            println!("Upper {} is the closest to {}", upper, start_time);
            closest_bus = busses[i];
            closest_diff = diff;
        }
    }

    println!("Closest Bus: {} - Diff: {} - Product: {}", closest_bus, closest_diff, closest_bus * closest_diff);
}

fn parse_input2(input: String) -> Vec<[u64; 2]> {
    let split: Vec<&str> = input.split("\n").collect();
    let busses: Vec<&str> = split[1].split(",").collect();

    let mut result: Vec<[u64; 2]> = [].to_vec();

    for i in 0..busses.len() {
        let bus = busses[i];
        match bus.parse::<u64>() {
            Ok(bus_id) => result.push([i as u64, bus_id]),
            Err(_) => continue,
        }
    }

    return result;
}

fn part2(input: String) {
    let bus_list = parse_input2(input);

    println!("Bus List: {:?}", bus_list);

    let n = get_num(bus_list);
    println!("N is: {}", n);
}

fn get_num(bus_list: Vec<[u64; 2]>) -> u64 {
    let mut curr: u64 = 17;

    // Increment by the cycles until we find one that matches
    let mut current_inc = 1;
    let mut bus_start = 1;

    let mut num_iter = 0;

    'outer: loop {
        num_iter += 1;
        for i in 0..bus_list.len() {
            let bus_id = bus_list[i][1];
            let bus_idx = bus_list[i][0];

            let offset = bus_idx;

            if (curr + offset) % bus_id != 0 {
                curr += current_inc;

                continue 'outer;
            } else {
                bus_start += 1;
                // we DID find a match
                current_inc = lcm(current_inc, bus_id);
                // println!("Start incrementing from {} by LCM current cycle: {}, {}", curr, current_inc, bus_id);
            }
        }

        println!("Iterations: {}", num_iter);
        return curr;
    }
}