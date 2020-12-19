use std::env;
use std::fs;
use std::collections::HashSet;

mod ticket;
use ticket::*;

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
    let (rules, my_ticket, other_tickets) = parse::input(input);

    println!("Rule Set:\n{:?}\n\nMy Ticket:\n{:?}\nOther Tickets:{:?}", rules, my_ticket, other_tickets);

    let mut sum = 0;

    for ticket in other_tickets {
        for value in ticket {
            if !is_valid_value(value, &rules) {
                println!("Invalid Number Found: {}", value);
                sum += value;
            }
        }
    }

    println!("Total Sum of Invalid Values: {}", sum);
}

fn part2(input: String) {
    let (rules, my_ticket, other_tickets) = parse::input(input);

    println!("Rule Set:\n{:?}\n\nMy Ticket:\n{:?}\nOther Tickets:{:?}", rules, my_ticket, other_tickets);

    let mut valid_tickets: Vec<Vec<u32>> = [].to_vec();

    'outer: for ticket in other_tickets {
        for value in ticket.clone() {
            if !is_valid_value(value, &rules) {
                println!("Invalid Number Found: {}", value);
                continue 'outer;
            }
        }

        valid_tickets.push(ticket.clone());
    }

    let mut positions = determine_rule_positions(valid_tickets, &rules);

    println!("Position: {:?}", positions);

    let mut sum: u64 = 0;
    for (idx, position) in positions.iter_mut().enumerate() {
        let mut curr = position.clone();
        if curr.len() < 9 {
            continue;
        }

        match &curr.as_mut_str()[..9] {
            "departure" => {
                println!("Adding {} to Sum: {} from idx: {} and value: {}", my_ticket[idx], sum, idx, curr);
                if sum == 0 {
                    sum = my_ticket[idx] as u64;
                } else {
                    sum = sum * my_ticket[idx] as u64;
                }
            },
            _ => (),
        }
    }

    println!("Sum: {}", sum);
}
