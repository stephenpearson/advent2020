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

fn copy(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let dim = (map.len(), map[0].len());
    let mut newmap: Vec<Vec<char>> = Vec::new();
    for i in 0..dim.0 {
        let mut row: Vec<char> = Vec::new();
        for j in 0..dim.1 {
            row.push(map[i][j]);
        }
        newmap.push(row);
    }
    newmap
}

fn getcell(map: &Vec<Vec<char>>, row: i32, col: i32) -> char {
    let dim = (map.len(), map[0].len());
    if row < 0 || col < 0 || row >= dim.0 as i32 || col >= dim.1 as i32 {
        return '.';
    }
    return map[row as usize][col as usize];
}

fn _print(map: &Vec<Vec<char>>) {
    let dim = (map.len(), map[0].len());
    for i in 0..dim.0 {
        for j in 0..dim.1 {
            print!("{}", map[i][j]);
        }
        println!("");
    }
}

fn occupied(map: &Vec<Vec<char>>) -> i32 {
    let dim = (map.len(), map[0].len());
    let mut c = 0;
    for i in 0..dim.0 {
        for j in 0..dim.1 {
            if map[i][j] == '#' {
                c += 1;
            }
        }
    }
    c
}

fn count1(map: &Vec<Vec<char>>, row: i32, col: i32) -> i32 {
    let mut c = 0;
    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 {
                continue;
            }
            let ch = getcell(&map, row + i, col + j);
            if ch == '#' {
                c += 1;
            }
        }
    }
    c
}

fn count2(map: &Vec<Vec<char>>, row: i32, col: i32) -> i32 {
    let dim = (map.len() as i32, map[0].len() as i32);
    let mut c = 0;
    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 {
                continue;
            }
            let mut cr = row + i;
            let mut cc = col + j;
            loop {
                if cr >= dim.0 || cr < 0 || cc >= dim.1 || cc < 0 {
                    break;
                }
                let ch = getcell(&map, cr, cc);
                if ch == '#' {
                    c += 1;
                    break;
                }
                if ch == 'L' {
                    break;
                }
                cr += i;
                cc += j;
            }
        }
    }
    c
}

fn iterate(
    map: &Vec<Vec<char>>,
    countfn: &dyn Fn(&Vec<Vec<char>>, i32, i32) -> i32,
    max: i32,
) -> (bool, Vec<Vec<char>>) {
    let dim = (map.len() as i32, map[0].len() as i32);
    let mut newmap = copy(map);
    let mut changed = false;
    for i in 0..dim.0 {
        for j in 0..dim.1 {
            let cur = getcell(&map, i, j);
            let c = countfn(&map, i, j);
            if cur == 'L' && c == 0 {
                newmap[i as usize][j as usize] = '#';
                changed = true;
            } else if cur == '#' && c >= max {
                newmap[i as usize][j as usize] = 'L';
                changed = true;
            }
        }
    }

    (changed, newmap)
}

fn main() {
    let mut map1: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(data) = line {
                let cols: Vec<char> = data.chars().collect();
                map1.push(cols);
            }
        }
    }
    let mut map2 = copy(&map1);

    loop {
        let tmp = iterate(&map1, &count1, 4);
        map1 = tmp.1;
        if tmp.0 == false {
            break;
        }
    }

    println!("");
    println!("occupied map1 = {}", occupied(&map1));

    loop {
        let tmp = iterate(&map2, &count2, 5);
        map2 = tmp.1;
        if tmp.0 == false {
            break;
        }
    }

    println!("occupied map2 = {}", occupied(&map2));
}
