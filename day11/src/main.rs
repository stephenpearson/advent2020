use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Copy, Clone, PartialEq)]
enum SeatStatus {
    Floor,
    EmptySeat,
    FullSeat,
}

impl SeatStatus {
    fn as_str(&self) -> &'static str {
        match *self {
            SeatStatus::Floor => ".",
            SeatStatus::EmptySeat => "L",
            SeatStatus::FullSeat => "#",
        }
    }
}

struct SeatMap<'a> {
    map: Vec<Vec<SeatStatus>>,
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
                result += format!("{}", self.map[i as usize][j as usize].as_str()).as_str();
            }
            result += "\n";
        }
        writeln!(f, "{}", result)
    }
}

impl<'a> SeatMap<'a> {
    fn new(
        map: &Vec<Vec<SeatStatus>>,
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

    fn parse_chars(input: Vec<char>) -> Vec<SeatStatus> {
        let mut result = Vec::new();
        for i in input {
            let v = match i {
                '.' => Some(SeatStatus::Floor),
                'L' => Some(SeatStatus::EmptySeat),
                '#' => Some(SeatStatus::FullSeat),
                _ => None,
            };
            if let Some(x) = v {
                result.push(x);
            }
        }
        result
    }

    fn getcell(&self, row: i32, col: i32) -> SeatStatus {
        if row < 0 || col < 0 || row >= self.rows || col >= self.cols {
            return SeatStatus::EmptySeat;
        }
        return self.map[row as usize][col as usize];
    }

    fn occupied(&self) -> i32 {
        let mut c = 0;
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.getcell(i, j) == SeatStatus::FullSeat {
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
                    if cur == SeatStatus::EmptySeat && c == 0 {
                        newmap[i as usize][j as usize] = SeatStatus::FullSeat;
                        changed = true;
                    } else if cur == SeatStatus::FullSeat && c >= self.max {
                        newmap[i as usize][j as usize] = SeatStatus::EmptySeat;
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
            if ch == SeatStatus::FullSeat {
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
                if ch == SeatStatus::FullSeat {
                    c += 1;
                    break;
                }
                if ch == SeatStatus::EmptySeat {
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
    let mut layout: Vec<Vec<SeatStatus>> = Vec::new();
    let file = File::open("./input.txt")?;
    for line in io::BufReader::new(file).lines() {
        if let Ok(data) = line {
            let cols: Vec<char> = data.chars().collect();
            layout.push(SeatMap::parse_chars(cols));
        }
    }
    let mut map1 = SeatMap::new(&layout, &count1, 4);
    let mut map2 = SeatMap::new(&layout, &count2, 5);

    println!("occupied map1 = {}", map1.iterate());
    println!("occupied map2 = {}", map2.iterate());
    Ok(())
}
