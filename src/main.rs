use std::{error::Error};

use crate::input::parse_input;

mod input;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let (grid, mut carts) = parse_input("input.txt")?;
    
    loop {
        // step each cart
        // (& check collision)

        carts.sort_by_key(|cart| (cart.curr_pos.y, cart.curr_pos.x));
        for i in 0..carts.len() {
            let cart = &mut carts[i];
            match cart.dirn {
                Dirn::North => cart.curr_pos.y -= 1,
                Dirn::South => cart.curr_pos.y += 1,
                Dirn::West => cart.curr_pos.x -= 1,
                Dirn::East => cart.curr_pos.x += 1,
            }

            match grid[cart.curr_pos.y][cart.curr_pos.x] {
                '+' => cart.handle_intersection(),
                '/' => cart.handle_slash(),
                '\\' => cart.handle_backslash(),
                _ => (),
            }

            let cart = &carts[i];
            for other in &carts {
                if cart.curr_pos == other.curr_pos {
                    println!("{},{}", cart.curr_pos.x, cart.curr_pos.y);
                    return Ok(());
                }
            }
        }
    }
}

impl Cart {
    fn handle_intersection(&mut self) {
        self.dirn = self.dirn.rotate(self.next_turn);
        self.next_turn = match self.next_turn {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        };
    }

    fn handle_slash(&mut self) {
        match self.dirn {
            Dirn::North | Dirn::South => self.dirn = self.dirn.rotate_cw(),
            Dirn::East | Dirn::West => self.dirn = self.dirn.rotate_ccw(),
        }
    }

    fn handle_backslash(&mut self) {
        match self.dirn {
            Dirn::North | Dirn::South => self.dirn = self.dirn.rotate_ccw(),
            Dirn::East | Dirn::West => self.dirn = self.dirn.rotate_cw(),
        }
    }
}

impl Dirn {
    fn rotate(self, turn: Turn) -> Dirn {
        match turn {
            Turn::Straight => self,
            Turn::Right => self.rotate_cw(),
            Turn::Left => self.rotate_ccw(),
        }
    }

    fn rotate_cw(self) -> Dirn {
        match self {
            Dirn::North => Dirn::East,
            Dirn::East => Dirn::South,
            Dirn::South => Dirn::West,
            Dirn::West => Dirn::North,
        }
    }

    fn rotate_ccw(self) -> Dirn {
        match self {
            Dirn::North => Dirn::West,
            Dirn::West => Dirn::South,
            Dirn::South => Dirn::East,
            Dirn::East => Dirn::North,
        }
    }
}

struct Cart {
    curr_pos: Point,
    dirn: Dirn,
    next_turn: Turn,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
enum Dirn {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy)]
enum Turn {
    Left,
    Straight,
    Right,
}
