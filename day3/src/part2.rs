use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let tree1 = determine_trees_in_path(1, 1).unwrap();
    let tree2 = determine_trees_in_path(3, 1).unwrap();
    let tree3 = determine_trees_in_path(5, 1).unwrap();
    let tree4 = determine_trees_in_path(7, 1).unwrap();
    let tree5 = determine_trees_in_path(1, 2).unwrap();

    println!("Tree1: {}, Tree2: {}, Tree3: {}, Tree4: {}, Tree5: {}", tree1, tree2, tree3, tree4, tree5);

    let trees = tree1 * tree2 * tree3 * tree4 * tree5;

    println!("Total Multiplied Trees: {}", trees);

    Ok(())
}

fn determine_trees_in_path(x_adjust: u32, y_adjust: u32) -> Result<u64, io::Error> {
  let file = File::open("./input.txt")?;
  let reader = BufReader::new(file);

  let mut x_pos: u32 = 0;
  let mut y_pos: u32 = 0;
  let mut trees: u64 = 0;

  for line in reader.lines() {

    if (y_pos % y_adjust != 0) {
      y_pos += 1;
      continue;
    }

    println!("CHECKING POSITION {} {}", x_pos, y_pos);
    let hit_tree = check_if_hit_tree(x_pos, line.unwrap());

    y_pos += 1;
    x_pos += x_adjust;

    if hit_tree {
      trees += 1;
    }
  }

  return Ok(trees);
}

fn check_if_hit_tree(x_pos: u32, line: String) -> bool {
  let mut chars: Vec<char> = line.chars().collect();

  let x = x_pos % 31;
  let mut hit = false;

  // println!("Checking x: {} in Line:\n{}", x, line);

  let x_idx = x as usize;

  if chars[x_idx] == '#' {
    // println!("HIT!");
    chars[x_idx] = 'X';
    hit = true;
  } else {
    chars[x_idx] = 'O';
  }

  let new_string: String = chars.iter().collect();
  // println!("{}", new_string);

  return hit;
}
