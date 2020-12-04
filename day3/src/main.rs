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

fn check_slope(map: &Vec<Vec<char>>, xinc: usize, yinc: usize) -> i32 {
    let mut count = 0;
    let mut x = 0;
    let mut y = 0;
    while y < map.len() - yinc {
        x = (x + xinc) % map[y].len();
        y += yinc;
        if map[y][x] == '#' {
            count += 1;
        }
    }
    count
}

fn main() {
    let mut map: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines("./map.txt") {
        for line in lines {
            if let Ok(data) = line {
                let tmp: Vec<char> = data.chars().collect();
                map.push(tmp);
            }
        }
    }
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut result: u64 = 0;
    for s in slopes.iter() {
        let count = check_slope(&map, s.0, s.1);
        println!("slope = {:?}, count = {}", s, count);
        if result == 0 {
            result = count as u64;
        } else {
            result *= count as u64;
        }
    }
    println!("result = {}", result);
}
