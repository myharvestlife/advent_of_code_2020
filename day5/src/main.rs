use std::env;
use std::fs;
use std::str;

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
    let rows = input.split("\n");

    let mut max = 0;

    for row in rows {
        if row == "" {
            println!("Empty Row");
            continue;
        }

        let seat_id = get_seat_id_from_row(row.to_string());
        if seat_id > max {
            max = seat_id;
        }
    }

    println!("MAX SEAT ID IS: {}", max);
}

fn part2(input: String) {
    let rows = input.split("\n");

    let mut seats: Vec<u32> =  Vec::new();

    for row in rows {
        if row == "" {
            println!("Empty Row");
            continue;
        }

        let seat_id = get_seat_id_from_row(row.to_string());
        seats.push(seat_id);
    }

    seats.sort();

    println!("Seats: {:?}", seats);

    let mut prev_seat = 0;
    let mut my_seat = 0;
    for seat in seats {
        if prev_seat != 0 && seat - 1 != prev_seat {
            my_seat = seat - 1;
        }

        prev_seat = seat;
    }

    println!("My Seat is: {}", my_seat);
}

fn get_seat_id_from_row(code: String) -> u32 {
    let first_part = &code[..7];
    let second_part = &code[7..];

    println!("First Part {} -- Second Part {}", first_part, second_part);
    let row = get_value_from_key('F', 'B', 0, 127, first_part);
    let column = get_value_from_key('L', 'R', 0, 7, second_part);

    return (row * 8) + column;
}

fn get_value_from_key(lower_key: char, upper_key: char, lower_bound: u8, upper_bound: u8, code: &str) -> u32 {
    let mut start: u8 = lower_bound;
    let mut end: u8 = upper_bound;

    for c in code.chars() {
        if end - start == 1 {
            if c == lower_key {
                println!("Returning START FOR {} - {}", c, start);
                return start.into();
            } else {
                println!("Returning END FOR {} - {}", c, end);
                return end.into();
            }
        }

        if c == lower_key {
            end = end - ((end - start + 1) / 2);
        } else if c == upper_key {
            start = start + ((end - start + 1) / 2);
        }

        println!("NEW RANGE <{}>: {} - {}", c, start, end);
    }

    return start.into();
}

