use std::env;
use std::fs;

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
    let mapping = input_converter::to_char_2d_vector(input);

    let new_mapping = process_mapping(mapping);

    // calculate the number
    let num_seated = get_num_seated(new_mapping);
    println!("Number Seated: {}", num_seated);
}

const FLOOR: char = '.';
const EMPTY: char = 'L';
const SEATED: char = '#';

fn process_mapping(mapping: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let (new_mapping, num_changes) = convert_map(mapping);

    if num_changes > 0 {
        println!("Num changes {} is greater than zero!", num_changes);
        return process_mapping(new_mapping);
    } else {
        return new_mapping;
    }
}

fn convert_map(mapping: Vec<Vec<char>>) -> (Vec<Vec<char>>, i32) {
    println!("First Row\n{:?}", mapping[0]);
    let mut new_map = mapping.clone();
    let mut changes = 0;

    for i in 0..mapping.len() {
        for j in 0..mapping[i].len() {
            let item = mapping[i][j];

            if item == FLOOR {
                continue;
            }

            if item == EMPTY {
                if seats_around(&mapping, i, j) == 0 {
                    new_map[i][j] = SEATED;
                    changes += 1;
                }
            }

            if item == SEATED {
                if seats_around(&mapping, i, j) >= 4 {
                    new_map[i][j] = EMPTY;
                    changes += 1;
                }
            }
        }
    }

    return (new_map, changes);
}

fn get_num_seated(mapping: Vec<Vec<char>>) -> i32 {
    let mut num_seated = 0;
    for i in 0..mapping.len() {
        for j in 0..mapping[i].len() {
            if mapping[i][j] == SEATED {
                num_seated += 1;
            }
        }
    }

    return num_seated;
}

fn seats_around(mapping: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut num_seats = 0;

    // Check WEST SEAT
    if j > 0 {
        if mapping[i][j - 1] == SEATED {
            num_seats += 1;
        }

        // CHECK NORTH-WEST SEAT
        if i > 0 {
            if mapping[i - 1][j - 1] == SEATED {
                num_seats += 1;
            }
        }

        // CHECK SOUTH-WEST SEAT
        if i < mapping.len() - 1 {
            if mapping[i + 1][j - 1] == SEATED {
                num_seats += 1;
            }
        }
    }

    // CHECK NORTH SEAT
    if i > 0 {
        if mapping[i - 1][j] == SEATED {
            num_seats += 1;
        }
    }

    // CHECK SOUTH SEAT
    if i < mapping.len() - 1 {
        if mapping[i + 1][j] == SEATED {
            num_seats += 1;
        }
    }

    if j < mapping[i].len() - 1 {
        // Check EAST SEAT
        if mapping[i][j + 1] == SEATED {
            num_seats += 1;
        }

        // CHECK NORTH-EAST SEAT
        if i > 0 {
            if mapping[i - 1][j + 1] == SEATED {
                num_seats += 1;
            }
        }

        // CHECK SOUTH-EAST SEAT
        if i < mapping.len() - 1 {
            if mapping[i + 1][j + 1] == SEATED {
                num_seats += 1;
            }
        }
    }

    // println!("Num Seats Next to {}x{} = {}", i, j, num_seats);
    return num_seats;
}

fn part2(input: String) {
    let mapping = input_converter::to_char_2d_vector(input);

    let new_mapping = process_mapping2(mapping);

    // calculate the number
    let num_seated = get_num_seated(new_mapping);
    println!("Number Seated: {}", num_seated);
}


fn process_mapping2(mapping: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let (new_mapping, num_changes) = convert_map2(mapping);

    if num_changes > 0 {
        println!("Num changes {} is greater than zero!", num_changes);
        return process_mapping2(new_mapping);
    } else {
        return new_mapping;
    }
}

fn convert_map2(mapping: Vec<Vec<char>>) -> (Vec<Vec<char>>, i32) {
    println!("First Row\n{:?}", mapping[0]);
    let mut new_map = mapping.clone();
    let mut changes = 0;

    for i in 0..mapping.len() {
        for j in 0..mapping[i].len() {
            let item = mapping[i][j];

            if item == FLOOR {
                continue;
            }

            if item == EMPTY {
                if seats_around2(&mapping, i, j) == 0 {
                    new_map[i][j] = SEATED;
                    changes += 1;
                }
            }

            if item == SEATED {
                if seats_around2(&mapping, i, j) >= 5 {
                    new_map[i][j] = EMPTY;
                    changes += 1;
                }
            }
        }
    }

    return (new_map, changes);
}

fn check_seat_dir(mapping: &Vec<Vec<char>>, i: usize, j: usize, i_incr: i32, j_incr: i32) -> bool {
  let mut x_loc = i as i32 + i_incr;
  let mut y_loc = j as i32 + j_incr;

  loop {
    // If this puts us out of bounds, there is no seat
    if x_loc < 0 || x_loc >= mapping.len() as i32 {
        return false;
    }

    if y_loc < 0 || y_loc >= mapping[i].len() as i32 {
        return false;
    }

    let item = mapping[x_loc as usize][y_loc as usize];

    if item == SEATED {
        return true;
    }

    if item == EMPTY {
        return false;
    }

    // if it's the floor - just increment again and try again
    if item == FLOOR {
        x_loc += i_incr;
        y_loc += j_incr;
        continue;
    }
  }
}

fn seats_around2(mapping: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut num_seats = 0;

    // Check WEST SEAT
    if check_seat_dir(mapping, i, j, -1, 0) {
        num_seats += 1;
    }

    // Check NORTH-WEST SEAT
    if check_seat_dir(mapping, i, j, -1, -1) {
        num_seats += 1;
    }

    // CHECK NORTH SEAT
    if check_seat_dir(mapping, i, j, 0, -1) {
        num_seats += 1;
    }

    // CHECK NORTH-EAST SEAT
    if check_seat_dir(mapping, i, j, 1, -1) {
        num_seats += 1;
    }

    // CHECK EAST SEAT
    if check_seat_dir(mapping, i, j, 1, 0) {
        num_seats += 1;
    }

    // CHECK SOUTH-EAST SEAT
    if check_seat_dir(mapping, i, j, 1, 1) {
        num_seats += 1;
    }

    // CHECK SOUTH SEAT
    if check_seat_dir(mapping, i, j, 0, 1) {
        num_seats += 1;
    }

    // CHECK SOUTH-WEST SEAT
    if check_seat_dir(mapping, i, j, -1, 1) {
        num_seats += 1;
    }

    // println!("Num Seats Next to {}x{} = {}", i, j, num_seats);
    return num_seats;
}
