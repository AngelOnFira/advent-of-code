use std::collections::HashMap;

use chumsky::{chain::Chain, text::digits};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Instruction {
    op: String,
    a: String,
    b: String,
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    // add z w
    // mod z 2
    // div w 2
    // add y w
    // mod y 2

    let re = Regex::new(r"^(\w+) (.*)$").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            // dbg!(&caps[2].split(' ').nth(1).unwrap());
            let mut split: Vec<&str> = caps[2].split(' ').collect();
            // dbg!(split.clone());
            let b = if split.len() > 1 {
                split.clone().iter().nth(1).unwrap().to_string()
            } else {
                "".to_string()
            };
            Instruction {
                op: caps[1].to_string(),
                a: split.iter().nth(0).unwrap().to_string(),
                b,
            }
        })
        .collect()
}

fn get_value(registers: [i64; 4], letter: String) -> i64 {
    match letter.as_str() {
        "w" => registers[0],
        "x" => registers[1],
        "y" => registers[2],
        "z" => registers[3],
        _ => 0,
    }
}

fn set_value(registers: &mut [i64; 4], letter: String, value: i64) {
    match letter.as_str() {
        "w" => registers[0] = value,
        "x" => registers[1] = value,
        "y" => registers[2] = value,
        "z" => registers[3] = value,
        _ => unreachable!(),
    }
}

#[aoc(day24, part1)]
pub fn solve_part1(input: &[Instruction]) -> i64 {
    // The ALU is a four-dimensional processing unit: it has integer variables
    // w, x, y, and z. These variables all start with the value 0. The ALU also
    // supports six instructions:

    // inp a - Read an input value and write it to variable a. add a b - Add the
    // value of a to the value of b, then store the result in variable a. mul a
    // b - Multiply the value of a by the value of b, then store the result in
    // variable a. div a b - Divide the value of a by the value of b, truncate
    // the result to an integer, then store the result in variable a. (Here,
    // "truncate" means to round the value toward zero.) mod a b - Divide the
    // value of a by the value of b, then store the remainder in variable a.
    // (This is also called the modulo operation.) eql a b - If the value of a
    // and b are equal, then store the value 1 in variable a. Otherwise, store
    // the value 0 in variable a.

    // In all of these instructions, a and b are placeholders; a will always be
    // the variable where the result of the operation is stored (one of w, x, y,
    // or z), while b can be either a variable or a number. Numbers can be
    // positive or negative, but will always be integers.

    // The ALU has no jump instructions; in an ALU program, every instruction is
    // run exactly once in order from top to bottom. The program halts after the
    // last instruction has finished executing.

    // (Program authors should be especially cautious; attempting to execute div
    // with b=0 or attempting to execute mod with a<0 or b<=0 will cause the
    // program to crash and might even damage the ALU. These operations are
    // never intended in any serious ALU program.)

    (0..99_999_999_999_999i64)
        .into_par_iter()
        .filter_map(|n| {
            let mut starting_num: i64 = 99_999_999_999_999 - n;
            if starting_num % 1_000_000 == 0 {
                println!("{}", starting_num);
            }

            // Split starting_num into digits
            let mut digits: Vec<i64> = Vec::new();
            let mut num = starting_num;
            while num > 0 {
                digits.push((num % 10) as i64);
                num /= 10;
            }

            // If zero is in digits, continue
            if digits.iter().any(|&x| x == 0) {
                starting_num -= 1;
                return None;
            }

            let mut digit_counter = 0;
            let mut variables: [i64; 4] = [0, 0, 0, 0];

            let mut legal = true;

            for instruction in input {
                let var2 = variables.clone();
                let a = get_value(variables, instruction.a.clone());

                let b = match instruction.b.parse::<i64>() {
                    Ok(val) => val,
                    Err(_) => get_value(variables, instruction.b.clone()),
                };

                match instruction.op.as_str() {
                    "add" => {
                        set_value(&mut variables, instruction.a.clone(), a + b);
                    }
                    "mul" => {
                        set_value(&mut variables, instruction.a.clone(), a * b);
                    }
                    "div" => {
                        if b == 0 {
                            legal = false;
                        } else {
                            set_value(&mut variables, instruction.a.clone(), a / b);
                        }
                    }
                    "mod" => {
                        if b == 0 {
                            legal = false;
                        } else {
                            set_value(&mut variables, instruction.a.clone(), a % b);
                        }
                    }
                    "eql" => {
                        set_value(
                            &mut variables,
                            instruction.a.clone(),
                            if a == b { 1 } else { 0 },
                        );
                    }
                    "inp" => {
                        // if digit_counter > 1 {
                        //     dbg!(starting_num, digit_counter);
                        // }
                        set_value(&mut variables, instruction.a.clone(), digits[digit_counter]);
                        digit_counter += 1;
                    }
                    _ => {
                        panic!("Unknown instruction: {}", instruction.op);
                    }
                }

                if !legal {
                    break;
                }
            }

            starting_num -= 1;

            // If z is zero, break
            if get_value(variables, "z".to_string()) == 0 && legal {
                println!("FOUND {}", starting_num);
                return Some(starting_num);
            }

            None
        })
        .count() as i64
}

#[aoc(day24, part2)]
pub fn solve_part2(input: &[Instruction]) -> i32 {
    3
}
