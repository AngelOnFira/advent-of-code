use super::intcode::intcode::{call_intcode, State};
use std::collections::HashMap;

#[aoc(day15, part1)]
pub fn solve_part1(input: &str) -> i128 {
    let mut panels: HashMap<String, i8> = HashMap::new();

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

    let mut color;
    let mut dir;

    let mut curr_color;
    let mut curr_dir = 0;
    let mut curr_pos_x = 0;
    let mut curr_pos_y = 0;

    println("{}" call_intcode(&mut state, true) as i8);

    12

    loop {
        curr_color = {
            match panels
                .get(&format!("{}:{}", curr_pos_x, curr_pos_y))
                .copied()
            {
                Some(color) => color,
                None => 0,
            }
        };

        state.input.push(curr_color as i128);

        color = call_intcode(&mut state, true) as i8;

        if color == -1 {
            break;
        }

        dir = call_intcode(&mut state, true);

        if dir == 0 {
            curr_dir -= 1;
            if curr_dir == -1 {
                curr_dir = 3
            }
        } else {
            curr_dir += 1;
            curr_dir %= 4;
        }

        panels.insert(format!("{}:{}", curr_pos_x, curr_pos_y), color);

        match curr_dir {
            0 => curr_pos_y -= 1,
            1 => curr_pos_x += 1,
            2 => curr_pos_y += 1,
            3 => curr_pos_x -= 1,
            _ => unreachable!(),
        }
    }

    return panels.keys().len() as i128;
}

#[cfg(test)]
mod tests {
    use super::solve_part1 as part1;
    // use super::solve_part2 as part2;
}
