use std::fs::File;
use std::io::{self, BufRead};

struct Ship {
    east: i32,
    south: i32,
    wp_east: i32,
    wp_north: i32,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            east: 0,
            south: 0,
            wp_east: 10,
            wp_north: 1,
        }
    }
    fn mhdistance(&self) -> i32 {
        let east = if self.east < 0 { -self.east } else { self.east };
        let south = if self.south < 0 {
            -self.south
        } else {
            self.south
        };
        east + south
    }
    fn apply(&mut self, input: &String) {
        let mut ch = input.chars();
        let command = ch.next().unwrap();
        let mut pstr = String::new();
        while let Some(x) = ch.next() {
            pstr += &x.to_string();
        }
        let param: i32 = pstr.parse().unwrap();

        match command {
            'N' => {
                self.wp_north += param;
            }
            'S' => {
                self.wp_north += -param;
            }
            'E' => {
                self.wp_east += param;
            }
            'W' => {
                self.wp_east += -param;
            }
            'R' => {
                for _ in 0..param / 90 {
                    let tmp = self.wp_east;
                    self.wp_east = self.wp_north;
                    self.wp_north = -tmp;
                }
            }
            'L' => {
                for _ in 0..param / 90 {
                    let tmp = self.wp_east;
                    self.wp_east = -self.wp_north;
                    self.wp_north = tmp;
                }
            }
            'F' => {
                self.east += self.wp_east * param;
                self.south += -self.wp_north * param;
            }
            _ => {
                panic!("unknown command");
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut ship = Ship::new();
    let file = File::open("./input.txt")?;
    for line in io::BufReader::new(file).lines() {
        if let Ok(data) = line {
            ship.apply(&data);
        }
    }
    println!("{}", ship.mhdistance());
    Ok(())
}
