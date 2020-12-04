use std::io::{self, BufRead};
use std::path::Path;
use std::{collections::HashMap, fs::File};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Passport {
    info: HashMap<String, String>,
}

impl<'a> Passport {
    fn new() -> Passport {
        Passport {
            info: HashMap::new(),
        }
    }
    fn set(&mut self, key: String, value: String) {
        self.info.insert(key, value);
    }
    fn parse_opt_string_to_i32(s: Option<&String>) -> Option<i32> {
        if let Some(opt) = s {
            if let Ok(v) = opt.parse::<i32>() {
                return Some(v);
            }
            None
        } else {
            None
        }
    }
    fn parse_string_to_i64(s: &String) -> Option<i64> {
        if let Ok(v) = s.parse::<i64>() {
            return Some(v);
        }
        None
    }
    fn check_numeric(&self, field: &str, min: i32, max: i32) -> bool {
        if let Some(v) = Passport::parse_opt_string_to_i32(self.info.get(field)) {
            if v >= min && v <= max {
                return true;
            }
        }
        false
    }
    fn check_height(&self) -> bool {
        if let Some(hgt) = self.info.get("hgt") {
            let len = hgt.len();
            if len < 3 {
                return false;
            }
            let unit = hgt[len - 2..].to_string();
            let num = hgt[..len - 2].to_string().parse::<i32>();
            if let Err(_) = num {
                return false;
            }
            let num_v = num.unwrap();
            if unit == "cm" {
                if num_v < 150 || num_v > 193 {
                    return false;
                }
            } else if unit == "in" {
                if num_v < 59 || num_v > 76 {
                    return false;
                }
            } else {
                return false;
            }
            true
        } else {
            false
        }
    }
    fn check_hcl(&self) -> bool {
        if let Some(hcl) = self.info.get("hcl") {
            let len = hcl.len();
            if len != 7 {
                return false;
            }
            let mut it = hcl.chars().into_iter();
            if let Some(x) = it.next() {
                if x != '#' {
                    return false;
                }
            } else {
                return false;
            }
            let valid_chars = vec![
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
            ];
            for _ in 0..6 {
                if let Some(x) = it.next() {
                    let mut ok = false;
                    for i in &valid_chars {
                        if x == *i {
                            ok = true;
                        }
                    }
                    if ok == false {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
    fn check_ecl(&self) -> bool {
        if let Some(ecl) = self.info.get("ecl") {
            let valid_ecl = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            let mut ok = false;
            for i in valid_ecl {
                if ecl == i {
                    ok = true;
                }
            }
            if ok == false {
                return false;
            }
            true
        } else {
            false
        }
    }
    fn check_pid(&self) -> bool {
        if let Some(pid) = self.info.get("pid") {
            if pid.len() != 9 {
                return false;
            }
            let tmp = Passport::parse_string_to_i64(pid);
            if let Some(v) = tmp {
                if v < 0 && v > 999999999 {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
    fn valid(&self) -> bool {
        let mandatory_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        for k in mandatory_keys {
            if !self.info.contains_key(k) {
                return false;
            }
        }
        if self.check_numeric("byr", 1920, 2002) == false {
            return false;
        }
        if self.check_numeric("iyr", 2010, 2020) == false {
            return false;
        }
        if self.check_numeric("eyr", 2020, 2030) == false {
            return false;
        }
        if self.check_height() == false {
            return false;
        }
        if self.check_hcl() == false {
            return false;
        }
        if self.check_ecl() == false {
            return false;
        }
        if self.check_pid() == false {
            return false;
        }
        true
    }
}

fn main() {
    let mut pp = Passport::new();
    let mut passports: Vec<Passport> = Vec::new();
    if let Ok(lines) = read_lines("./passports.txt") {
        for line in lines {
            if let Ok(data) = line {
                if data == "" {
                    passports.push(pp);
                    pp = Passport::new();
                } else {
                    let items: Vec<&str> = data.split(' ').collect();
                    for i in items {
                        let split: Vec<&str> = i.split(':').collect();
                        pp.set(split[0].to_string(), split[1].to_string());
                    }
                }
            }
        }
    }
    passports.push(pp);
    let mut count = 0;
    for p in passports {
        if p.valid() {
            count += 1;
        }
    }
    println!("valid = {}", count);
}
