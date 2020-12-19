use std::collections::HashSet;

#[derive(Debug)]
pub struct Rule {
  pub name: String,
  pub range_a: [u32; 2],
  pub range_b: [u32; 2],
}

pub fn create_matcher(rules: &Vec<Rule>) -> HashSet<String> {
  let mut set = HashSet::new();

  for rule in rules {
    // Initialize Every One to True
    set.insert(rule.name.clone());
  }

  return set;
}

pub fn is_valid_value(val: u32, rules: &Vec<Rule>) -> bool {
  for rule in rules {
    if val >= rule.range_a[0] && val <= rule.range_a[1] {
      return true;
    }

    if val >= rule.range_b[0] && val <= rule.range_b[1] {
      return true;
    }
  }

  return false;
}

// Returns the rule names in idx order
pub fn determine_rule_positions(tickets: Vec<Vec<u32>>, rules: &Vec<Rule>) -> Vec<String> {
  let mut matcher_list: Vec<HashSet<String>> = [].to_vec();

  for _ in 0..20 {
    matcher_list.push(create_matcher(rules));
  }


  'outer: for ticket in tickets {
    // println!("Checking Ticket: {:?}", ticket);
    for (idx, val) in ticket.iter().enumerate() {
      for rule in rules {
        let exists_in_rule = matcher_list[idx].contains(&rule.name);
        if !exists_in_rule {
          // If we have already removed this rule from this index - just skip
          // println!("Could not locate {} in {:?}", rule.name, matcher_list[idx]);
          continue;
        }

        if (*val >= rule.range_a[0] && *val <= rule.range_a[1]) || (*val >= rule.range_b[0] && *val <= rule.range_b[1]) {
          // println!("Value {} falls in rule {} range", val, rule.name.clone());
        } else {
          // println!("Value {} does not match rule {} for idx: {} --- deleting rule from idx", val, rule.name.clone(), idx);
          matcher_list[idx].remove(&rule.name);

          if matcher_list[idx].len() == 1 {
            println!("List at {} is len 1", idx);
            let to_remove = matcher_list[idx].iter().next().unwrap().clone();
            // Remove this one from all other lists
            matcher_list.iter_mut().enumerate().for_each(|(i, list)| {
              if i != idx {
                let removed = list.remove(&to_remove);

                if removed { println!("Removed {}", to_remove); }
              }
            });
          }
        }
      }

      if all_are_one(&matcher_list) {
        println!("Found rule matches: {:?}", matcher_list);
        break 'outer;
      }
    }
  }

  matcher_list = clean_list(&mut matcher_list);

  println!("RULE MATCHES: {:?}", matcher_list);

  return matcher_list.iter().map(|r| r.iter().next().unwrap().clone()).collect();
}

fn all_are_one(list: &Vec<HashSet<String>>) -> bool {
  for item in list.iter() {
    if item.len() != 1 {
      return false;
    }
  }

  return true;
}

fn is_clean(matcher_list: &Vec<HashSet<String>>) -> bool {
  for list in matcher_list {
    if list.len() != 1 {
      return false;
    }
  }

  return true;
}

fn clean_list(matcher_list: &mut Vec<HashSet<String>>) -> Vec<HashSet<String>> {
  let mut cleaned: HashSet<String> = HashSet::new();

  while !is_clean(&matcher_list) {

    let mut to_clean: Option<String> = None;
    let mut idx_found: Option<usize> = None;
    for (idx, list) in matcher_list.iter_mut().enumerate() {
      if list.len() == 1 && !cleaned.contains(list.iter().next().unwrap()) {
        to_clean = Some(list.iter().next().unwrap().clone());
        idx_found = Some(idx);
        break;
      }
    }

    match to_clean {
      Some(val) => {
        println!("Cleaning {}", val);
        cleaned.insert(val.clone());
        for (idx, clean_list) in matcher_list.iter_mut().enumerate() {
          if idx != idx_found.unwrap() {
            clean_list.remove(&val);
          }
        }
      },
      None => (),
    }
  }

  return matcher_list.clone();
}

pub mod parse {
  use super::Rule;

  pub fn input(input: String) -> (Vec<Rule>, Vec<u32>, Vec<Vec<u32>>) {
    // Split into pieces
    let parts: Vec<&str> = input.split("\n\n").filter(|n| n.len() > 0).collect();
    let rule_string = parts[0];
    let my_ticket_string: Vec<&str> = parts[1].split("\n").collect();
    let mut all_ticket_strings: Vec<&str> = parts[2].split("\n").collect();

    let rules = rule_string.split("\n").map(|r| rule(r)).collect();
    let my_ticket: Vec<u32> = ticket(my_ticket_string[1]);

    all_ticket_strings.remove(0);
    let other_tickets: Vec<Vec<u32>> = all_ticket_strings.iter().filter(|n| n.len() > 0).map(|ts| ticket(ts)).collect();

    return (rules, my_ticket, other_tickets);
  }

  pub fn ticket(input: &str) -> Vec<u32> {
    input.split(",").map(|t| t.parse().unwrap()).collect()
  }

  pub fn rule(input: &str) -> Rule {
    let halves: Vec<&str> = input.split(": ").collect();
    let name = halves[0];
    let rules: Vec<&str> = halves[1].split(" or ").collect();
    let rule_a: Vec<u32> = rules[0].split("-").map(|r| r.parse().unwrap()).collect();
    let rule_b: Vec<u32> = rules[1].split("-").map(|r| r.parse().unwrap()).collect();

    let rule = Rule {
      name: name.to_string(),
      range_a: [rule_a[0], rule_a[1]],
      range_b: [rule_b[0], rule_b[1]],
    };

    return rule;
  }
}

