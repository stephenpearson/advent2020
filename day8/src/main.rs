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
struct Instruction {
    opcode: String,
    operand: i32,
    executed: bool,
}

impl Instruction {
    fn new(opcode: String, operand: i32) -> Instruction {
        Instruction {
            opcode,
            operand,
            executed: false,
        }
    }
}

fn execute(program: &mut Vec<Instruction>) -> (bool, i32) {
    let mut pc: i32 = 0;
    let mut acc = 0;
    loop {
        if pc as usize >= program.len() {
            return (true, acc);
        }
        let inst = &mut program[pc as usize];
        if inst.executed {
            break;
        }
        inst.executed = true;
        match inst.opcode.as_str() {
            "jmp" => {
                pc += inst.operand;
                continue;
            }
            "acc" => {
                acc += inst.operand;
            }
            "nop" => {}
            _ => {
                println!("Unknown instruction");
            }
        }
        pc += 1;
    }
    (false, acc)
}

fn main() {
    let mut program: Vec<Instruction> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(data) = line {
                let fields: Vec<&str> = data.split(" ").collect();
                let operand = fields[1].parse();
                program.push(Instruction::new(fields[0].to_string(), operand.unwrap()));
            }
        }
    }

    for i in 0..program.len() {
        for x in &mut program {
            x.executed = false;
        }
        let orig = program[i].opcode.clone();
        if program[i].opcode == "jmp" {
            program[i].opcode = "nop".to_string();
        } else if program[i].opcode == "nop" {
            program[i].opcode = "jmp".to_string();
        }

        let result = execute(&mut program);
        if result.0 {
            println!("correct = {}, acc = {}", result.0, result.1);
            break;
        }
        program[i].opcode = orig;
    }
}
