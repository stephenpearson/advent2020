use std::io::{self, BufRead};
use std::{collections::HashMap, fs::File};

struct System {
    mask1: u64,
    floating: u64,
    fcount: u32,
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
            result.push(match v {
                'X' => x,
                '1' => one,
                '0' => zero,
                _ => '?',
            });
        }
        i64::from_str_radix(&result, 2).unwrap() as u64
    }
    fn count(s: &str) -> u32 {
        s.chars().fold(0, |a, v| a + if v == 'X' { 1 } else { 0 })
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

        for i in 0..2u64.pow(self.fcount) {
            let mut maddr = (addr | self.mask1) & self.floating;
            let mut shift = self.floating;
            let mut pos: u64 = 1;
            let mut bcount: u64 = 0;
            for p in 0..64 {
                if shift & 1 == 0 {
                    let bit = (i & pos) << (p - bcount);
                    maddr = maddr | bit;
                    bcount += 1;
                    pos = pos << 1;
                }
                shift = shift >> 1;
            }
            self.memory.insert(maddr, value);
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
        self.memory.values().sum()
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
