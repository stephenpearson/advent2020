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
fn main() {
    let mut numbers = Vec::<i32>::new();
    if let Ok(lines) = read_lines("./list.txt") {
        for line in lines {
            if let Ok(data) = line {
                let n: i32 = data.parse().unwrap();
                numbers.push(n);
            }
        }
    }
    println!("{:?}", numbers);
    for i in 0..numbers.len() - 2 {
        for j in i..numbers.len() - 1 {
            for k in j..numbers.len() {
                let a = numbers[i];
                let b = numbers[j];
                let c = numbers[k];

                if a + b + c == 2020 {
                    println!("{} + {} + {} = {}", a, b, c, a + b + c);
                    println!("{} x {} x {} = {}", a, b, c, a * b * c);
                }
            }
        }
    }
}
