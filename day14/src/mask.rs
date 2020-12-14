use regex::Regex;

pub mod parser {
  pub fn is_mask(line: String) -> bool {
    line[0..4] == *"mask"
  }

  pub fn get_mask(line: String) -> String {
    let re = Regex::new("^mask = ([10X]+)$").unwrap();

    re.captures(line).unwrap().get(1).map_or("", |m| m.as_str())
  }

  pub fn get_mem(line: String) -> (u64, u64) {
    let re = Regex::new("^mem\\[([0-9]+)\\] = ([0-9]+)").unwrap();

    let location = re.captures(line).unwrap().get(1).map_or(0, |m| m.parse());
    let value = re.captures(line).unwrap().get(2).map_or(0, |m| m.parse());

    (location, value)
  }
}

pub struct Memory {
  pub memory: [u64; 36],
  pub mask: Vec<char>,
}

