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

// Kernighan's bit counting algorithm
fn count_bits(mut n: u32) -> i32 {
    let mut count = 0;
    while n != 0 {
        n = n & (n - 1);
        count += 1;
    }
    count
}

fn main() {
    let index = 'a' as usize;
    let mut gval1: u32 = 0;
    let mut gval2: Option<u32> = None;
    let mut sum1 = 0;
    let mut sum2 = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(data) = line {
                if data == "" {
                    sum1 += count_bits(gval1);
                    if let Some(x) = gval2 {
                        sum2 += count_bits(x);
                    }
                    gval1 = 0;
                    gval2 = None;
                } else {
                    let tmp = data
                        .chars()
                        .map(|x| 1 << x as usize - index)
                        .fold(0, |a, x| a | x);
                    gval1 |= tmp;
                    if let Some(x) = gval2 {
                        gval2 = Some(x & tmp);
                    } else {
                        gval2 = Some(tmp);
                    }
                }
            }
        }
        sum1 += count_bits(gval1);
        if let Some(x) = gval2 {
            sum2 += count_bits(x);
        }
    }
    println!("sum1 = {}", sum1);
    println!("sum2 = {}", sum2);
}
