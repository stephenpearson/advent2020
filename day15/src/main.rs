use std::collections::HashMap;

fn main() {
    let numbers = vec![19, 20, 14, 0, 9, 1];
    let mut last: HashMap<i32, (i32, i32)> = HashMap::new();
    let mut cur = 0;
    for turn in 0..30000000 {
        if turn < numbers.len() as i32 {
            cur = numbers[turn as usize];
            last.insert(cur, (turn, turn));
        } else {
            let l = last.entry(cur).or_insert((turn, turn));
            cur = (*l).0 - (*l).1;
            let tmp = last.entry(cur).or_insert((turn, turn));
            *tmp = (turn, (*tmp).0);
        }
    }
    println!("cur = {}", cur);
}
