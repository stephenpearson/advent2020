use std::collections::HashMap;
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

fn parse_line(words: Vec<&str>) -> Vec<(i32, String)> {
    let mut result = Vec::<(i32, String)>::new();
    let mut i = 4;
    while i < words.len() {
        if words[i] == "no" {
            break;
        }
        let count = words[i].parse().unwrap();
        let colour = format!("{} {}", words[i + 1], words[i + 2]);
        result.push((count, colour));
        i += 4;
    }
    result
}

fn contains_colour(bags: &HashMap<String, Vec<(i32, String)>>, colour: &str, search: &str) -> bool {
    if let Some(contents) = bags.get(colour) {
        for bag in contents {
            if bag.1 == search || contains_colour(&bags, &bag.1, search) {
                return true;
            }
        }
    } else {
        println!("No bag info for {}", colour);
    }
    false
}

fn count_bags(bags: &HashMap<String, Vec<(i32, String)>>, colour: &str) -> i32 {
    let mut count = 0;
    if let Some(contents) = bags.get(colour) {
        for bag in contents {
            count += bag.0 + bag.0 * count_bags(&bags, &bag.1);
        }
    } else {
        println!("No bag info for {}", colour);
    }
    count
}

fn main() {
    let mut bags = HashMap::<String, Vec<(i32, String)>>::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(data) = line {
                let words: Vec<&str> = data.split(" ").collect();
                let colour = format!("{} {}", words[0], words[1]);
                bags.insert(colour, parse_line(words));
            }
        }
    }

    let mut count = 0;
    for i in &bags {
        if contains_colour(&bags, &i.0, "shiny gold") {
            println!("{}", i.0);
            count += 1;
        }
    }
    println!("bags containing shiny gold bags = {}", count);

    let contained_bags = count_bags(&bags, "shiny gold");
    println!("shiny gold bags contain {} bags", contained_bags);
}
