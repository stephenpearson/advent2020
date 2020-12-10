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

fn iter(l: &Vec<i32>, a: i32, t: i32) -> i32 {
    if a == t {
        return 1;
    }
    let mut count = 0;
    for i in 1..4 {
        if l.contains(&(a + i)) {
            count += iter(&l, a + i, t);
        }
    }
    count
}

fn main() {
    let mut l = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(data) = line {
                let num = data.parse::<i32>().unwrap();
                l.push(num);
            }
        }
    }
    l.sort();
    let last = l[l.len() - 1];
    l.push(last + 3);
    let mut a = 0;
    let mut nodes = Vec::new();
    for i in &l {
        let gap = *i - a;
        if gap == 3 {
            nodes.push(*i);
        }
        a = *i;
    }
    a = 0;
    let mut product: i64 = 0;
    for i in nodes {
        let tmp = iter(&l, a, i);
        a = i;
        if product == 0 {
            product = tmp as i64;
        } else {
            product *= tmp as i64;
        }
    }
    println!("{}", product);
}
