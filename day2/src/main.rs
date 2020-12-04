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

#[derive(Debug)]
struct Rule {
    range_from: i32,
    range_to: i32,
    letter: char,
    text: String,
}

impl Rule {
    fn new_from_str(s: String) -> Rule {
        let fields: Vec<&str> = s.split(' ').collect();
        let range: Vec<&str> = fields[0].split('-').collect();
        let ch: Vec<&str> = fields[1].split(':').collect();
        Rule {
            range_from: range[0].parse().unwrap(),
            range_to: range[1].parse().unwrap(),
            letter: ch[0].chars().next().unwrap(),
            text: fields[2].to_string(),
        }
    }
    fn is_valid(&self) -> bool {
        let mut count = 0;
        for i in self.text.chars() {
            if i == self.letter {
                count += 1;
            }
        }
        if count >= self.range_from && count <= self.range_to {
            true
        } else {
            false
        }
    }
    fn is_valid2(&self) -> bool {
        let charlist: Vec<char> = self.text.chars().collect();
        let p1 = charlist[(self.range_from as usize) - 1];
        let p2 = charlist[(self.range_to as usize) - 1];
        if (p1 == self.letter && p2 != self.letter) || (p1 != self.letter && p2 == self.letter) {
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut count1 = 0;
    let mut count2 = 0;
    if let Ok(lines) = read_lines("./list.txt") {
        for line in lines {
            if let Ok(data) = line {
                let r = Rule::new_from_str(data);
                if r.is_valid() {
                    count1 += 1;
                }
                if r.is_valid2() {
                    count2 += 1;
                }
            }
        }
    }
    println!("count1 = {}", count1);
    println!("count2 = {}", count2);
}
