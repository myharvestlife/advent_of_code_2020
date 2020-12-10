use std::env;
use std::fs;
use regex::Regex;
use std::collections::HashMap;

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

#[derive(Clone)]
struct Bag {
    color: String,
    children: Vec<String>,
}

fn part1(input: String) {
    let bag_map = generate_bag_map(input, false);

    let mut total = 0;

    for bag in bag_map.values() {
        if bag_has_gold_eventually(bag, &bag_map) {
            println!("{} eventually has shiny gold", bag.color);
            total += 1;
        }
    }

    println!("TOTAL {}", total);
}

fn bag_has_gold_eventually(bag: &Bag, bag_map: &HashMap<String, Bag>) -> bool {
    for child in bag.children.clone() {
        if child == "shiny gold" {
            return true;
        }

        let child_bag = bag_map.get(&child).unwrap();

        if bag_has_gold_eventually(child_bag, bag_map) {
            return true;
        }
    }

    return false;
}

fn count_children_bags(bag: &Bag, bag_map: &HashMap<String, Bag>) -> u32 {
    let mut total = 0;
    for child in bag.children.clone() {
        if child == "" {
            continue;
        }
        total += 1;

        let child_bag = bag_map.get(&child).unwrap();
        let children_total = count_children_bags(child_bag, bag_map);

        total += children_total;
    }

    return total;
}

fn generate_bag_map(input: String, multi: bool) -> HashMap<String, Bag> {
    let mut map = HashMap::new();
    let lines: Vec<&str> = input.split("\n").collect();
    for line in lines {
        if line == "" {
            continue;
        }

        let bag = get_bag(line, multi);
        println!("Bag: {} - {:?}", bag.color, bag.children);
        map.insert(bag.color.clone(), bag);

    }

    return map;
}


fn get_bag(line: &str, multi: bool) -> Bag {
    let name_regex = Regex::new(r"^([a-z]+ [a-z]+) bags contain (.*)$").unwrap();

    let captures = name_regex.captures(line).unwrap();

    let name = captures.get(1).map_or("", |m| m.as_str());
    let latter = captures.get(2).map_or("", |m| m.as_str());

    // println!("Line: {}", line);
    // println!("Captures: {} -- {}", name, latter);

    if latter == "no other bags." {
        return Bag { color: String::from(name), children: [].to_vec() };
    } else {
        if multi {
            let children = get_multi_bags_from_latter(latter);
            return Bag { color: String::from(name), children };
        } else {
            let children = get_bags_from_latter(latter);
            return Bag { color: String::from(name), children };
        }
    }
}

fn get_bags_from_latter(latter: &str) -> Vec<String> {
    let bags_regex = Regex::new(r"\d+ ([a-z]+ [a-z]+) bags?").unwrap();

    let captures = bags_regex.captures_iter(latter);
    let mut bags: Vec<String> = [].to_vec();
    for cap in captures {
        let bag = cap.get(1).map_or("", |m| m.as_str());
        // println!("Match {:?}", bag);
        bags.push(bag.to_string());
    }

    return bags;
}


fn get_multi_bags_from_latter(latter: &str) -> Vec<String> {
    let bags_regex = Regex::new(r"(\d+) ([a-z]+ [a-z]+) bags?").unwrap();

    let captures = bags_regex.captures_iter(latter);
    let mut bags: Vec<String> = [].to_vec();
    for cap in captures {
        let num_bags: u32 = cap.get(1).map_or(0, |m| m.as_str().parse().unwrap());
        let bag = cap.get(2).map_or("", |m| m.as_str());
        // println!("Match {:?}", bag);
        let mut i = 0;
        while i < num_bags {
            i += 1;
            bags.push(bag.to_string());
        }
    }

    return bags;
}

fn part2(input: String) {
    let bag_map = generate_bag_map(input, true);

    let starting_bag = bag_map.get("shiny gold").unwrap();
    let total = count_children_bags(starting_bag, &bag_map);

    println!("TOTAL {}", total);
}