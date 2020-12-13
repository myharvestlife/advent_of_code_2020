use std::env;
use std::fs;

use utils::input_converter;

mod directions;
use directions::*;

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

    let mut ship = Ship {
        x: 0,
        y: 0,
        dir: DirectionFacing::EAST,
    };

    for line in lines {
        let (command, amount) = parser::direction(line);

        move_ship(&mut ship, command, amount);
    }

    println!("Ships Final Position: {} x {}  -- Distance: {}", ship.x, ship.y, ship.x.abs() + ship.y.abs());
}

fn part2(input: String) {
    let lines = input_converter::to_string_vec(input);

    let mut ship = SuperShip {
        x: 0,
        y: 0,
        way_x: 10,
        way_y: 1,
    };

    for line in lines {
        let (command, amount) = parser::direction(line);

        println!("Command: {}, amount: {}", command, amount);
        move_ship2(&mut ship, command, amount);
        println!("SHIP: {:?}", ship);
    }

    println!("Ships Final Position: {} x {}  -- Distance: {}", ship.x, ship.y, ship.x.abs() + ship.y.abs());
}