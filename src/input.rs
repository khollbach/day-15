use std::fs;

use crate::{Cart, Dirn, Point, Result, Turn};

pub fn parse_input(filename: &str) -> Result<(Vec<Vec<char>>, Vec<Cart>)> {
    let lines = fs::read_to_string(filename)?;

    let num_rows = lines.lines().count();
    let num_cols = lines.lines().next().unwrap().len();

    let mut carts = vec![];

    let mut cells = vec![vec![' '; num_cols]; num_rows];
    for (y, line) in lines.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if "^>v<".contains(c) {
                let cart = Cart {
                    curr_pos: Point { x, y },
                    dirn: Dirn::from_char(c),
                    next_turn: Turn::Left,
                    crashed: false,
                };

                cells[y][x] = match cart.dirn {
                    Dirn::North | Dirn::South => '|',
                    Dirn::East | Dirn::West => '-',
                };

                carts.push(cart);
            } else {
                cells[y][x] = c;
            }
        }
    }

    Ok((cells, carts))
}

impl Dirn {
    fn from_char(c: char) -> Dirn {
        match c {
            '^' => Dirn::North,
            '>' => Dirn::East,
            'v' => Dirn::South,
            '<' => Dirn::West,
            _ => panic!("not a dirn: {c}"),
        }
    }
}
