use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("./input.txt")?;
    let mut lines = io::BufReader::new(file).lines();
    lines.next();
    let bstr = lines.next().unwrap().unwrap();
    let bstrvec: Vec<&str> = bstr.split(",").collect();
    let mut busses: Vec<Option<i64>> = Vec::new();
    for b in bstrvec {
        let p = b.parse();
        if let Ok(x) = p {
            busses.push(Some(x));
        } else {
            busses.push(None);
        }
    }

    let start = 1;
    let mut time = start;
    let mut inc = 1;
    let mut found = 0;
    loop {
        let mut fail = true;
        println!("Checking time = {}", time);
        let mut fc = 0;
        let mut cfc = 0;
        for (i, optbus) in busses.iter().enumerate() {
            if let Some(bus) = optbus {
                let t = time + i as i64;
                let result = t % bus;

                println!("{} % {} = {}", t, bus, result);
                if result == 0 {
                    fc += 1;
                    if fail && fc > cfc {
                        cfc = fc;
                    }
                } else {
                    fc = 0;
                    fail = false;
                }
            }
        }
        if cfc > found {
            found = cfc;
            inc = 1;
            let mut i = 0;
            let mut p = 0;
            while p < found {
                if let Some(x) = busses[i] {
                    inc *= x;
                    p += 1;
                }
                i += 1;
            }
            println!("found = {}", found);
            println!("set inc to = {}", inc);
        }
        if fail {
            println!("{}", time);
            break;
        }
        time += inc;
    }
    Ok(())
}
