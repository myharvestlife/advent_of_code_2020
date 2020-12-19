
use std::collections::HashMap;

pub struct Memory {
  pub num_map: HashMap<u32, u32>,
  pub curr_idx: u32,
  pub next_num: u32,
}

pub fn preload(input: Vec<u32>, mem: &mut Memory) {
  for (idx, val) in input.iter().enumerate() {
    mem.num_map.insert(*val, idx as u32 + 1);
    mem.curr_idx = idx as u32 + 1;
    mem.next_num = 0;
  }

  println!("Hash Map: {:?} -- {} -- {}", mem.num_map, mem.curr_idx, mem.next_num);
}

pub fn process(mem: &mut Memory, amount: u32) -> u32 {

  loop {
    mem.curr_idx += 1;
    // println!("TURN {}", mem.curr_idx);

    // println!("Value: {}", mem.next_num);
    // println!("Map: {:?}", mem.num_map);
    let new_num = match mem.num_map.get(&mem.next_num) {
      Some(v) => {
        // println!("LAST IDX IS: {}", v);
        mem.curr_idx - v
      },
      None => 0,
    };

    // println!("New Number at IDX {} should be {}", mem.curr_idx, new_num);


    if mem.curr_idx == amount {
      return mem.next_num;
    }

    mem.num_map.insert(mem.next_num, mem.curr_idx);
    mem.next_num = new_num;
  }
}

