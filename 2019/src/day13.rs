use super::intcode::intcode::{call_intcode, State};
use std::collections::HashMap;

#[aoc(day13, part1)]
pub fn solve_part1(input: &str) -> i128 {
    // let panels: HashMap<String, i8> = HashMap::new();

    let mut state = State {
        instruction_pointer: 0,
        relative_pointer: 0,
        instructions: Default::default(),
        input: Vec::new(),
        output: Vec::new(),
        memory: HashMap::new(),
    };

    state.instructions = input
        .split(",")
        .map(|input| input.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();

    // state.input.push(curr_color as i128);
    state.memory.insert("0".to_string(), 2);

    let mut block_count = 0;
    let mut x;

    loop {
        x = call_intcode(&mut state, true);
        if x == -1 {
            return block_count;
        }

        let _ = call_intcode(&mut state, true);

        if call_intcode(&mut state, true) == 2 {
            block_count += 1;
        }

    }
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &str) -> i128 {
    // let panels: HashMap<String, i8> = HashMap::new();

    let mut state = State {
        instruction_pointer: 0,
        relative_pointer: 0,
        instructions: Default::default(),
        input: Vec::new(),
        output: Vec::new(),
        memory: HashMap::new(),
    };

    state.instructions = input
        .split(",")
        .map(|input| input.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();

    state.input.push(-1);
    state.input.push(0);
    state.input.push(0);
    state.input.push(0);
    state.input.push(0);
    state.input.push(0);
    state.input.push(0);
    state.input.push(0);

    state.memory.insert("0".to_string(), 2);

    let mut block_count = 0;
    let mut x;

    loop {
        x = call_intcode(&mut state, true);
        if x == -1 {
            return block_count;
        }

        let y = call_intcode(&mut state, true);

        match call_intcode(&mut state, true) {
            2 => block_count += 1,
            i => {
                if x == -1 && y == 0 {
                    println!("{}", i);
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::solve_part1 as part1;
    // use super::solve_part2 as part2;
}
