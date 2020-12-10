use std::env;
use std::fs;

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


fn convert_input(input: String) -> Vec<u64> {
    return input.split("\n").map(|n| n.parse().unwrap()).collect();
}

fn check_if_sum_of_prev(previous_values: Vec<u64>, current_value: u64) -> bool {
    // Make sure len is always 25
    assert_eq!(previous_values.len(), 25);

    println!("Checking if {} is a sum of the previous 25 values", current_value);
    println!("Prev Values: {:?}", previous_values);

    let mut i = 0;
    while i < 25 {

        let mut j = 0;
        while j < 25 {
            println!("I {} J {}", i, j);
            println!("Checking if {} + {} = {}", previous_values[i], previous_values[j], current_value);

            if i != j && previous_values[i] + previous_values[j] == current_value {
                return true;
            }

            j += 1;
        }

        i += 1;
    }

    return false;
}

fn part1(input: String) {
    let data = convert_input(input);

    let mut start = 0;
    let mut end = 25;

    loop {
        println!("Creating slice from {} to {}", start, end);
        if !check_if_sum_of_prev(data[start..end].to_vec().clone(), data[end]) {
            break;
        }

        start += 1;
        end += 1;
    }

    println!("IS NOT SUM OF PREV 25 VALUES: {}", data[end]);
}

fn part2(input: String) {
    const CHECK_NUMBER: u64 = 14360655;

    let data = convert_input(input);

    let mut start = 0;
    let mut end = 1;
    let mut sum = data[0];

    loop {
        while sum < CHECK_NUMBER {
            sum += data[end];
            end += 1;
        }

        if sum == CHECK_NUMBER {
            println!("CHECK NUMBER FOUND FROM RANGE: {} to {}", start, end);
            break;
        }

        while sum > CHECK_NUMBER {
            sum -= data[start];
            start += 1;
        }
    }

    assert_eq!(CHECK_NUMBER, data.as_slice()[start..end].to_vec().into_iter().fold(0, |t, v| t + v));

    let min = data.as_slice()[start..end].to_vec().into_iter().fold(9999999999999999999, |least, current| if current < least { current } else { least });
    let max = data.as_slice()[start..end].to_vec().into_iter().fold(0, |most, current| if current > most { current } else { most });

    println!("Start idx: {}, End idx: {}, Min: {} Max: {} Sum: {}", start, end, min, max, min + max);
}