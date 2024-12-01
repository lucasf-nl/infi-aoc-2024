mod asm;

use std::collections::HashMap;
use std::fs::File;
use asm::*;
use crate::cloudmap::{CondensedCoordinate, Coordinate};

pub fn part1() -> (HashMap<CondensedCoordinate, i32>, i32) {
    let instructions = parse_asm(File::open("instructions.asm").unwrap());
    let mut cloud_map: HashMap<CondensedCoordinate, i32> = HashMap::new();
    let mut global_counter = 0;

    for (x, y, z) in (0..30).flat_map(|x| {
        (0..30).flat_map(move |y| {
            (0..30).map(move |z| (x, y, z))
        })
    }) {
        let mut stack: Vec<i32> = Vec::new();
        let mut program_counter: usize = 0;

        let resolve_value = |v: &Value| -> i32 {
            match v {
                Value::X => x,
                Value::Y => y,
                Value::Z => z,
                Value::Num(v) => *v,
            }
        };

        loop {
            let instruction = instructions.get(program_counter);

            // A possible optimization here is to evaluate which instructions happen
            // most and sort this match based on that to lower the amount of missed branches.
            match instruction {
                Some(Instruction::Add) => {
                    if stack.len() < 2 {
                        panic!("Cannot add when stack is less than 2");
                    }

                    // In rust a value cannot be borrowed as mutable more than once in one statement,
                    // because of this the add instruction has to be 3 statements.
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();

                    stack.push(v1 + v2);
                },
                Some(Instruction::Ret) => {
                    if stack.len() > 0 {
                        let value = stack.pop().unwrap();
                        global_counter += value;

                        let coordinate = Coordinate::from((x as u8, y as u8, z as u8));
                        let condensed = CondensedCoordinate::from(coordinate);

                        cloud_map.insert(condensed, value);

                        break;
                    }
                },
                Some(Instruction::Push(v)) => {
                    stack.push(resolve_value(v));
                }
                Some(Instruction::Jmpos(v)) => {
                    if stack.len() < 1 {
                        panic!("Cannot jmpos when stack is less than 1");
                    }

                    if stack.pop().unwrap() >= 0 {
                        program_counter += resolve_value(v) as usize;
                    }
                },
                None => {
                    panic!("No instruction at PC");
                }
            }

            program_counter += 1;
        }
    }

    println!("Total {}", global_counter);

    #[cfg(test)]
    assert_eq!(global_counter, 4726);

    (cloud_map, global_counter)
}

#[cfg(test)]
fn test_part1() {
    part1();
}