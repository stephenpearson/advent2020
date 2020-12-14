use std::io::{self, BufRead};
use std::{collections::HashMap, fs::File};

struct System {
    mask1: u64,
    floating: u64,
    fcount: u64,
    memory: HashMap<u64, u64>,
}

impl System {
    fn new() -> System {
        System {
            mask1: 0,
            floating: 0,
            fcount: 0,
            memory: HashMap::<u64, u64>::new(),
        }
    }
    fn replace(s: &str, x: char, one: char, zero: char) -> u64 {
        let mut result = String::new();
        for v in s.chars() {
            if v == 'X' {
                result.push(x);
            } else if v == '1' {
                result.push(one);
            } else if v == '0' {
                result.push(zero);
            }
        }
        i64::from_str_radix(&result, 2).unwrap() as u64
    }
    fn count(s: &str) -> u64 {
        let mut count = 0;
        for v in s.chars() {
            if v == 'X' {
                count += 1;
            }
        }
        count
    }
    fn set_mask(&mut self, maskstr: &str) {
        self.mask1 = System::replace(maskstr, '0', '1', '0');
        self.floating = System::replace(maskstr, '0', '1', '1');
        self.fcount = System::count(maskstr);
    }
    fn set_mem(&mut self, memstr: &str, valuestr: &str) {
        let tmp = String::from(memstr);
        let split_tmp: Vec<&str> = tmp[..tmp.len() - 1].split("[").collect();
        let addr: u64 = split_tmp[1].parse().unwrap();
        let value: u64 = valuestr.parse().unwrap();

        let maddr = addr | self.mask1;
        let ptr = self.memory.entry(maddr).or_insert(value);
        *ptr = value;

        for i in 0..2u64.pow(self.fcount as u32) {
            let mut maddr = (addr | self.mask1) & self.floating;
            let mut shift = self.floating;
            let mut pos: u64 = 1;
            let mut bcount: u64 = 0;
            for p in 0..64 {
                if shift & 1 == 0 {
                    let bit = (i & pos) << (p - bcount);
                    bcount += 1;
                    maddr = (maddr | bit) & 68719476735;
                    pos = pos << 1;
                }
                shift = shift >> 1;
            }
            println!("set {} with {}", maddr, value);
            let ptr = self.memory.entry(maddr).or_insert(value);
            *ptr = value;
        }
    }
    fn apply(&mut self, line: String) {
        let fields: Vec<&str> = line.split(" ").collect();
        let cmd = fields[0];
        let param = fields[2];
        if cmd == "mask" {
            self.set_mask(param);
        } else {
            self.set_mem(cmd, param);
        }
    }
    fn sum(&self) -> u64 {
        let mut sum = 0;
        for (_, v) in &self.memory {
            sum += v;
        }
        sum
    }
}

fn main() -> std::io::Result<()> {
    let mut sys = System::new();
    let file = File::open("./input.txt")?;
    for line in io::BufReader::new(file).lines() {
        if let Ok(data) = line {
            sys.apply(data);
        }
    }
    println!("sum = {}", sys.sum());
    Ok(())
}
