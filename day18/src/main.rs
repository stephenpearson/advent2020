use std::fs::File;
use std::io::{self, BufRead};

fn apply(acc: Option<i64>, op: char, num: i64) -> i64 {
    if acc.is_none() {
        return num;
    }
    match op {
        ' ' => num,
        '+' => acc.unwrap() + num,
        _ => 0,
    }
}

fn parse(ch: &mut std::str::Chars) -> Option<i64> {
    let mut op: char = ' ';
    let mut left: Option<i64> = None;
    let mut products: Vec<Option<i64>> = Vec::new();
    while let Some(x) = ch.next() {
        match x {
            ')' => {
                break;
            }
            ' ' => {
                continue;
            }
            '+' | '*' => {
                op = x;
            }
            _ => {
                let right;
                right = if x == '(' {
                    parse(ch).unwrap()
                } else {
                    x.to_digit(10).unwrap() as i64
                };
                if x == '(' || ('0'..='9').contains(&x) {
                    if op == '+' || op == ' ' {
                        left = Some(apply(left, op, right));
                    } else {
                        products.push(left);
                        left = Some(right);
                    }
                }
            }
        }
    }
    products.push(left);
    let result: i64 = products.iter().flatten().product();
    Some(result)
}

fn main() -> std::io::Result<()> {
    let mut sum: i64 = 0;
    let file = File::open("./input.txt")?;
    for line in io::BufReader::new(file).lines() {
        if let Ok(data) = line {
            let mut ch = data.chars();
            let result = parse(&mut ch).unwrap();
            sum += result;
        }
    }
    println!("{}", sum);
    Ok(())
}
