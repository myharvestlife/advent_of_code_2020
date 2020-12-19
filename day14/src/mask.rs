use std::collections::HashMap;

pub mod parser {
  use regex::Regex;

  pub fn is_mask(line: String) -> bool {
    line[0..4] == *"mask"
  }

  pub fn get_mask(line: String) -> String {
    let re = Regex::new("^mask = ([10X]+)$").unwrap();

    re.captures(&line[..]).unwrap().get(1).map_or("".to_string(), |m| m.as_str().to_string())
  }

  pub fn get_mem(line: String) -> (u64, u64) {
    let re = Regex::new("^mem\\[([0-9]+)\\] = ([0-9]+)").unwrap();

    let location = re.captures(&line[..]).unwrap().get(1).map_or(0, |m| m.as_str().parse().unwrap());
    let value = re.captures(&line[..]).unwrap().get(2).map_or(0, |m| m.as_str().parse().unwrap());

    (location, value)
  }
}
pub struct Memory {
  pub memory: HashMap<u64, u64>,
  pub mask: Vec<char>,
}

impl Memory {
  pub fn get_total(&self) -> u64 {
    let mut total = 0;

    for val in self.memory.values() {
      total += val;
    }

    return total;
  }
}

pub fn update_memory(mem: &mut Memory, loc: u64, val: u64) {
  let mem_string: Vec<char> = format!("{:b}", val).chars().rev().collect();

  println!("Mem String: {:?}", mem_string);
  let mut result: Vec<char> = ['0'; 36].to_vec();

  for i in 0..mem.mask.len() {
    if mem.mask[i] == '1' {
      result[i] = '1';
    } else if mem.mask[i] == '0' {
      result[i] = '0';
    } else {
      result[i] = match mem_string.get(i) {
        Some(val) => *val,
        None => '0',
      }
    }
  }

  let memory_value: u64 = result.iter().enumerate().fold(0, |mut sum, (idx, val)| {
    if *val == '1' {
      sum += (2 as u64).pow(idx as u32);
    }

    sum
  });

  mem.memory.insert(loc, memory_value);
}

fn get_mem_locations(mask: &Vec<char>, loc: u64) -> Vec<u64> {
  let mut locations: Vec<Vec<char>> = [].to_vec();
  let loc_string: Vec<char> = format!("{:b}", loc).chars().rev().collect();

  locations.push([].to_vec());

  for i in 0..mask.len() {
    if mask[i] == '1' {
      for loca in locations.iter_mut() {
        loca.push('1');
      }
    } else if mask[i] == '0' {
      for loca in locations.iter_mut() {
        loca.push(match loc_string.get(i) {
          Some(val) => *val,
          None => '0',
        });
      }
    } else {
      // Floating 'X'
      let mut new_locas: Vec<Vec<char>> = [].to_vec();
      for loca in locations.iter_mut() {
        let mut new_loca = loca.clone();
        loca.push('0');
        new_loca.push('1');

        new_locas.push(new_loca);
      }

      for l in new_locas {
        locations.push(l);
      }
    }
  }

  // println!("Write To Locations: {:?}", locations);

  return locations.iter().map(|l| l.iter().enumerate().fold(0, |mut sum, (idx, val)| {
    if *val == '1' {
      sum += (2 as u64).pow(idx as u32);
    }

    sum
  })).collect();
}

pub fn update_memory2(mem: &mut Memory, loc: u64, val: u64) {
  let memory_locations = get_mem_locations(&mem.mask, loc);

  // println!("Write to locations: {:?}", memory_locations);

  for mem_loc in memory_locations {
    mem.memory.insert(mem_loc, val);
  }
}

