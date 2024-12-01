use std::fs::File;
use std::io::{BufRead, BufReader};

#[repr(u8)]
pub enum Instruction {
    Push(Value),
    Add,
    Jmpos(Value),
    Ret
}

#[repr(u8)]
pub enum Value {
    X,
    Y,
    Z,
    Num(i32)
}

pub fn parse_asm(file: File) -> Vec<Instruction> {
    let mut reader = BufReader::new(file);

    let mut instructions = Vec::<Instruction>::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let split = line.split_whitespace().collect::<Vec<&str>>();
                let instruction = split[0];

                let instruction = match instruction {
                    "push" => {Instruction::Push(parse_value(split))},
                    "jmpos" => {Instruction::Jmpos(parse_value(split))},
                    "add" => Instruction::Add,
                    "ret" => Instruction::Ret,
                    _ => unreachable!()
                };

                instructions.push(instruction);
            },
            Err(err) => {
                println!("Issue reading line: {}", err);
            }
        }
    }

    println!("Parsed {} instructions", instructions.len());

    instructions
}

fn parse_value(instruction: Vec<&str>) -> Value {
    if instruction.len() < 2 {
        panic!("Instruction must have at least two parts for there to be a value.");
    }

    match instruction[1].to_lowercase().as_ref() {
        "x" => Value::X,
        "y" => Value::Y,
        "z" => Value::Z,
        _ => Value::Num(instruction[1].parse::<i32>().unwrap()),
    }
}