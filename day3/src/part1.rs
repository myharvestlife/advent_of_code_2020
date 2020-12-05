use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let mut x_pos: usize = 0;
    let mut trees: u32 = 0;

    for line in reader.lines() {
      let hit_tree = check_if_hit_tree(x_pos, line.unwrap());

      x_pos += 3;

      if hit_tree {
        trees += 1;
      }
    }

    println!("Hit {} Trees", trees);

    Ok(())
}

fn check_if_hit_tree(x_pos: usize, line: String) -> bool {
  let mut chars: Vec<char> = line.chars().collect();

  let x = x_pos % 31;
  let mut hit = false;

  // println!("Checking x: {} in Line:\n{}", x, line);

  if chars[x] == '#' {
    // println!("HIT!");
    chars[x] = 'X';
    hit = true;
  } else {
    chars[x] = 'O';
  }

  let new_string: String = chars.iter().collect();
  println!("{}", new_string);

  return hit;
}
