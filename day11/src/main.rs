use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};

struct SeatMap<'a> {
    map: Vec<Vec<char>>,
    rows: i32,
    cols: i32,
    countfn: &'a dyn Fn(&SeatMap, i32, i32) -> i32,
    max: i32,
}

impl<'a> fmt::Display for SeatMap<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for i in 0..self.rows {
            for j in 0..self.cols {
                result += format!("{}", self.map[i as usize][j as usize]).as_str();
            }
            result += "\n";
        }
        write!(f, "{}", result)
    }
}

impl<'a> SeatMap<'a> {
    fn new(
        map: &Vec<Vec<char>>,
        countfn: &'a dyn Fn(&SeatMap, i32, i32) -> i32,
        max: i32,
    ) -> SeatMap<'a> {
        SeatMap::<'a> {
            map: map.clone(),
            rows: map.len() as i32,
            cols: map[0].len() as i32,
            countfn,
            max,
        }
    }

    fn getcell(&self, row: i32, col: i32) -> char {
        if row < 0 || col < 0 || row >= self.rows || col >= self.cols {
            return '.';
        }
        return self.map[row as usize][col as usize];
    }

    fn occupied(&self) -> i32 {
        let mut c = 0;
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.getcell(i, j) == '#' {
                    c += 1;
                }
            }
        }
        c
    }

    fn iterate(&mut self) -> i32 {
        loop {
            let mut newmap = self.map.clone();
            let mut changed = false;
            for i in 0..self.rows {
                for j in 0..self.cols {
                    let cur = self.getcell(i, j);
                    let c = (self.countfn)(&self, i, j);
                    if cur == 'L' && c == 0 {
                        newmap[i as usize][j as usize] = '#';
                        changed = true;
                    } else if cur == '#' && c >= self.max {
                        newmap[i as usize][j as usize] = 'L';
                        changed = true;
                    }
                }
            }
            self.map = newmap;
            if changed == false {
                break;
            }
        }
        self.occupied()
    }
}

fn count1(map: &SeatMap, row: i32, col: i32) -> i32 {
    let mut c = 0;
    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 {
                continue;
            }
            let ch = map.getcell(row + i, col + j);
            if ch == '#' {
                c += 1;
            }
        }
    }
    c
}

fn count2(map: &SeatMap, row: i32, col: i32) -> i32 {
    let mut c = 0;
    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 {
                continue;
            }
            let mut cr = row + i;
            let mut cc = col + j;
            loop {
                if cr >= map.rows || cr < 0 || cc >= map.cols || cc < 0 {
                    break;
                }
                let ch = map.getcell(cr, cc);
                if ch == '#' {
                    c += 1;
                    break;
                }
                if ch == 'L' {
                    break;
                }
                cr += i;
                cc += j;
            }
        }
    }
    c
}

fn main() -> std::io::Result<()> {
    let mut layout: Vec<Vec<char>> = Vec::new();
    let file = File::open("./input.txt")?;
    for line in io::BufReader::new(file).lines() {
        if let Ok(data) = line {
            let cols: Vec<char> = data.chars().collect();
            layout.push(cols);
        }
    }
    let mut map1 = SeatMap::new(&layout, &count1, 4);
    let mut map2 = SeatMap::new(&layout, &count2, 5);

    println!("occupied map1 = {}", map1.iterate());
    println!("occupied map2 = {}", map2.iterate());
    Ok(())
}
