use core::ops::Range;
use std::io::{self, BufRead, BufReader, Lines};
use std::{collections::HashMap, fs::File};

#[derive(Debug)]
struct Ticket {
    numbers: Vec<i32>,
}

impl Ticket {
    fn new(str: String) -> Ticket {
        let ns: Vec<&str> = str.split(",").collect();
        let mut list = Vec::new();
        for i in ns {
            let parsed: i32 = i.parse().unwrap();
            list.push(parsed);
        }
        Ticket { numbers: list }
    }
    fn valid(&self, rules: &Vec<Rule>) -> bool {
        'outer: for number in &self.numbers {
            for rule in rules {
                if rule.range1.contains(number) || rule.range2.contains(number) {
                    continue 'outer;
                }
            }
            return false;
        }
        true
    }
    fn invalid_values(&self, rules: &Vec<Rule>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        'outer: for number in &self.numbers {
            for rule in rules {
                if rule.range1.contains(number) || rule.range2.contains(number) {
                    continue 'outer;
                }
            }
            result.push(*number);
        }
        result
    }
}

#[derive(Debug)]
struct Rule {
    name: String,
    range1: Range<i32>,
    range2: Range<i32>,
}

impl Rule {
    fn new(str: String) -> Rule {
        let fields: Vec<&str> = str.split(":").collect();
        let ranges: Vec<&str> = fields[1].split(" or ").collect();
        Rule {
            name: fields[0].to_string(),
            range1: Rule::parse_range_str(&ranges[0][1..]),
            range2: Rule::parse_range_str(ranges[1]),
        }
    }
    fn parse_range_str(str: &str) -> Range<i32> {
        let fields: Vec<&str> = str.split("-").collect();
        let start: i32 = fields[0].parse().unwrap();
        let end: i32 = fields[1].parse().unwrap();
        start..end + 1
    }
}

fn load_rules(lines: &mut Lines<BufReader<File>>) -> Vec<Rule> {
    let mut rules: Vec<Rule> = Vec::new();
    loop {
        let line = lines.next();
        let data = line.unwrap().unwrap();
        if data == "" {
            break;
        }
        rules.push(Rule::new(data));
    }
    rules
}

fn load_tickets(lines: &mut Lines<BufReader<File>>) -> Vec<Ticket> {
    let mut tickets: Vec<Ticket> = Vec::new();
    for line in lines {
        let data = line.unwrap();
        tickets.push(Ticket::new(data));
    }
    tickets
}

fn load_data(filename: &str) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let file = File::open(filename).unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let rules = load_rules(&mut lines);

    lines.next();
    let myticket = Ticket::new(lines.next().unwrap().unwrap());

    lines.next();
    lines.next();
    let tickets = load_tickets(&mut lines);
    (rules, myticket, tickets)
}

fn part1(tickets: &Vec<Ticket>, rules: &Vec<Rule>) {
    let mut sum = 0;
    for t in tickets {
        let tmp: i32 = t.invalid_values(rules).into_iter().sum();
        sum += tmp;
    }
    println!("part1 = {}", sum);
}

fn get_rulepositions(tickets: &Vec<Ticket>, rules: &Vec<Rule>) -> HashMap<usize, Vec<i32>> {
    let valid_tickets: Vec<&Ticket> = tickets.into_iter().filter(|x| x.valid(&rules)).collect();
    let mut rulepositions: HashMap<usize, Vec<i32>> = HashMap::new();
    for (ri, rule) in rules.iter().enumerate() {
        'outer: for pos in 0..rules.len() {
            for ticket in &valid_tickets {
                let number = ticket.numbers[pos];
                if !rule.range1.contains(&number) && !rule.range2.contains(&number) {
                    continue 'outer;
                }
            }
            let rp = rulepositions.entry(ri).or_insert(Vec::new());
            rp.push(pos as i32);
        }
    }
    rulepositions
}

fn part2(myticket: &Ticket, tickets: &Vec<Ticket>, rules: &Vec<Rule>) {
    let rulepositions = get_rulepositions(&tickets, &rules);

    let mut part2: i64 = 1;
    let mut found: Vec<i32> = Vec::new();
    while found.len() < 20 {
        for r in &rulepositions {
            let tmp: Vec<&i32> = r.1.into_iter().filter(|x| !found.contains(x)).collect();
            if tmp.len() == 1 {
                let pos = r.0;
                let val = *tmp[0];
                let rule = &rules[*pos];
                if rule.name.starts_with("departure") {
                    part2 = part2 * myticket.numbers[val as usize] as i64;
                }
                found.push(val);
            }
        }
    }
    println!("part2 = {}", part2);
}

fn main() -> std::io::Result<()> {
    let (rules, myticket, tickets) = load_data("./input.txt");

    part1(&tickets, &rules);
    part2(&myticket, &tickets, &rules);
    Ok(())
}
