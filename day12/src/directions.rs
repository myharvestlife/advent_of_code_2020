
pub mod parser {
  pub fn direction(dir: String) -> (char, i32) {
    return (dir[0..1].chars().next().unwrap(), dir[1..].parse().unwrap());
  }
}

#[derive(PartialEq)]
pub enum DirectionFacing {
  NORTH,
  SOUTH,
  EAST,
  WEST,
}

pub struct Ship {
  pub x: i32,
  pub y: i32,
  pub dir: DirectionFacing,
}

#[derive(Debug)]
pub struct SuperShip {
  pub x: i32,
  pub y: i32,
  pub way_x: i32,
  pub way_y: i32,
}

pub fn move_ship(ship: &mut Ship, command: char, amount: i32) {
  if command == NORTH {
    ship.y += amount;
    return;
  }

  if command == SOUTH {
    ship.y -= amount;
    return;
  }

  if command == EAST {
    ship.x += amount;
    return;
  }

  if command == WEST {
    ship.x -= amount;
    return;
  }

  if command == FORWARD {
    move_ship_forward(ship, amount);
    return;
  }

  if command == LEFT || command == RIGHT {
    rotate_ship(ship, command, amount);
  }
}

pub fn move_ship2(ship: &mut SuperShip, command: char, amount: i32) {
  if command == NORTH {
    ship.way_y += amount;
    return;
  }

  if command == SOUTH {
    ship.way_y -= amount;
    return;
  }

  if command == EAST {
    ship.way_x += amount;
    return;
  }

  if command == WEST {
    ship.way_x -= amount;
    return;
  }

  if command == FORWARD {
    move_ship_to_waypoint(ship, amount);
    return;
  }

  if command == LEFT || command == RIGHT {
    rotate_waypoint(ship, command, amount);
    return;
  }
}

fn move_ship_to_waypoint(ship: &mut SuperShip, amount: i32) {
  for _ in 0..amount {
    ship.x += ship.way_x;
    ship.y += ship.way_y;
  }
}

fn rotate_waypoint(ship: &mut SuperShip, command: char, amount: i32) {
  let rotations = amount / 90;

  for _ in 0..rotations {
    let mut new_way_x = ship.way_x;
    let mut new_way_y = ship.way_y;

    if command == LEFT {
      new_way_x = ship.way_y * -1;
      new_way_y = ship.way_x;
    }

    if command == RIGHT {
      new_way_x = ship.way_y;
      new_way_y = ship.way_x * -1;
    }

    ship.way_x = new_way_x;
    ship.way_y = new_way_y;
  }
}

fn move_ship_forward(ship: &mut Ship, amount: i32) {
  match ship.dir {
    DirectionFacing::EAST => ship.x += amount,
    DirectionFacing::WEST => ship.x -= amount,
    DirectionFacing::NORTH => ship.y += amount,
    DirectionFacing::SOUTH => ship.y -= amount,
  }
}

fn rotate_ship(ship: &mut Ship, command: char, amount: i32) {
  let rotations = amount / 90;

  if command == LEFT {
    for _ in 0..rotations {
      rotate_left(ship);
    }
  }

  if command == RIGHT {
    for _ in 0..rotations {
      rotate_right(ship);
    }
  }
}

fn rotate_left(ship: &mut Ship) {
  match ship.dir {
    DirectionFacing::NORTH => ship.dir = DirectionFacing::WEST,
    DirectionFacing::WEST => ship.dir = DirectionFacing::SOUTH,
    DirectionFacing::SOUTH => ship.dir = DirectionFacing::EAST,
    DirectionFacing::EAST => ship.dir = DirectionFacing::NORTH,
  }
}

fn rotate_right(ship: &mut Ship) {
  match ship.dir {
    DirectionFacing::NORTH => ship.dir = DirectionFacing::EAST,
    DirectionFacing::WEST => ship.dir = DirectionFacing::NORTH,
    DirectionFacing::SOUTH => ship.dir = DirectionFacing::WEST,
    DirectionFacing::EAST => ship.dir = DirectionFacing::SOUTH,
  }
}

pub const LEFT: char = 'L';
pub const RIGHT: char = 'R';
pub const FORWARD: char = 'F';

pub const NORTH: char = 'N';
pub const SOUTH: char = 'S';
pub const EAST: char = 'E';
pub const WEST: char = 'W';