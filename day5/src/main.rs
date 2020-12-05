use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn seat_id(input: &str) -> (u16, u16, u16) {
    let mut result: u16 = 0;
    for i in input.chars() {
        result = result << 1;
        if i == 'B' || i == 'R' {
            result |= 0x1;
        } else {
            result |= 0x0;
        }
    }
    (result, result & 0x3f8, result & 0x7)
}
fn main() {
    let mut max = 0;
    let mut seats = HashMap::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(data) = line {
                let (id, row, seat) = seat_id(&data);
                if id > max {
                    max = id;
                }
                seats.entry(row).or_insert(Vec::<u16>::new());
                if let Some(x) = seats.get_mut(&row) {
                    x.push(seat);
                }
            }
        }
    }
    println!("max = {}", max);
    for (r, rowseats) in seats {
        if rowseats.len() == 7 {
            for s in 0..8 {
                if rowseats.get(s).is_none() {
                    println!("missing seat = {}", r + s as u16);
                }
            }
        }
    }
}
