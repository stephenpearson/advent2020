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

fn verify(list: &Vec<i64>, length: usize) -> Option<i64> {
    let mut p = length;
    while p < list.len() {
        let mut found = false;
        for i in p - length..p - 1 {
            for j in i + 1..p {
                let iv = list[i];
                let jv = list[j];
                if iv + jv == list[p] && iv != jv {
                    found = true;
                }
            }
        }
        if found == false {
            return Some(list[p]);
        }
        p += 1;
    }
    None
}

fn contiguous(list: &Vec<i64>, target: i64) -> Option<i64> {
    let l = list.len();
    for start in 0..l - 1 {
        for end in (start + 1)..l {
            let mut sum = 0;
            for i in start..end {
                sum += list[i];
            }
            if sum == target {
                let mut smallest = list[start];
                let mut largest = list[start];
                for i in start..end {
                    if list[i] > largest {
                        largest = list[i];
                    }
                    if list[i] < smallest {
                        smallest = list[i];
                    }
                }
                return Some(smallest + largest);
            }
        }
    }
    None
}

fn main() {
    let mut input = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(data) = line {
                input.push(data.parse::<i64>().unwrap());
            }
        }
    }
    let inv = verify(&input, 25).unwrap();
    println!("invalid = {}", inv);
    let cnt = contiguous(&input, inv).unwrap();
    println!("contiguous = {}", cnt);
}
