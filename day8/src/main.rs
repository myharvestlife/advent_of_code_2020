use std::env;
use std::fs;
use std::collections::HashSet;
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
    let mut acc = 0;
    let mut line = 0;
    let mut lines_visited: HashSet<usize> = HashSet::new();

    let finished = false;

    let code: Vec<&str> = input.split("\n").collect();

    while !finished {
        // Break on Duplicated Instruction
        if lines_visited.contains(&line) {
            break;
        }

        lines_visited.insert(line);

        let instruction = get_instruction(code.get(line).unwrap());

        let command = instruction.0;
        let value = instruction.1;

        if command == "acc" {
            acc += value;
            line += 1;
            continue;
        }

        if command == "nop" {
            line += 1;
            continue;
        }

        if command == "jmp" {
            if value > 0 {
                line += value.abs() as usize;
            } else if value < 0 {
                line -= value.abs() as usize;
            }

            continue;
        }
    }

    println!("ACC Value is: {}", acc);
}

fn get_instruction(code_line: &str) -> (&str, i32) {
    if code_line == "" {
        return ("end", 0);
    }

    let instruction_regex = Regex::new(r"^(acc|jmp|nop) ([-+]\d+)$").unwrap();
    let captures = instruction_regex.captures(code_line).unwrap();
    let command = captures.get(1).map_or("", |m| m.as_str());
    let command_val = captures.get(2).map_or(0, |m| m.as_str().parse::<i32>().unwrap());

    println!("Command: {} - Value: {}", command, command_val);

    return (command, command_val);
}

fn fix_line(line_no: usize, code: &Vec<&str>) -> (i32, String) {
    let instruction = get_instruction(code.get(line_no).unwrap());

    let command = instruction.0;
    let command_val = instruction.1;

    let mut new_command = String::from("");

    if command == "jmp" {
        new_command = String::from("nop");
    } else if command == "nop" {
        new_command = String::from("jmp");
    }

    return (command_val, new_command);
}

fn part2(input: String) {
    let mut acc = 0;
    let mut line = 0;
    let mut prev_line = 0;
    let mut lines_visited: HashSet<usize> = HashSet::new();

    let mut finished = false;

    let mut code: Vec<&str> = input.split("\n").collect();

    while !finished {
        let instruction = get_instruction(code.get(line).unwrap());
        let command = instruction.0.to_string();
        let value = instruction.1;

        lines_visited.insert(line);

        if command == "acc" {
            acc += value;
            line += 1;
            continue;
        }

        if command == "nop" {
            let new_set = lines_visited.clone();
            let new_line = adjust_line(line, value);

            let attempt = try_from_line(code.clone(), new_set, new_line, acc);
            if attempt.0 == true {
                println!("SUCCESS!");
                acc = attempt.1;
                break;
            }

            line += 1;
            continue;
        }

        if command == "jmp" {
            let new_set = lines_visited.clone();
            let new_line = line + 1;
            let attempt = try_from_line(code.clone(), new_set, new_line, acc);
            if attempt.0 == true {
                println!("SUCCESS!");
                acc = attempt.1;
                break;
            }


            line = adjust_line(line, value);
            continue;
        }


        if command == "end" {
            println!("HIT LAST LINE! Ending!");
            break;
        }
    }

    println!("FINAL ACC: {}", acc);
}

fn adjust_line(line: usize, value: i32) -> usize {
    let mut line = line;

    if value > 0 {
        line += value.abs() as usize;
    } else if value < 0 {
        line -= value.abs() as usize;
    }

    return line;
}

fn try_from_line(code: Vec<&str>, lines_visited: HashSet<usize>, start_at: usize, acc: i32) -> (bool, i32) {
    let mut acc = acc;
    let mut line = start_at;
    let mut lines_visited = lines_visited.clone();

    loop {
        // Try to Fix previous line and retry
        if lines_visited.contains(&line) {
            println!("VISITED LINE");
            return (false, acc);
        }

        lines_visited.insert(line);

        let instruction = get_instruction(code.get(line).unwrap());

        let command = instruction.0.to_string();
        let value = instruction.1;


        if command == "acc" {
            acc += value;
            line += 1;
            continue;
        }

        if command == "nop" {
            line += 1;
            continue;
        }

        if command == "jmp" {
            if value > 0 {
                line += value.abs() as usize;
            } else if value < 0 {
                line -= value.abs() as usize;
            }

            continue;
        }

        if command == "end" {
            println!("HIT LAST LINE! Ending!");
            break;
        }
    }

    println!("ACC Value is: {}", acc);
    return (true, acc);
}