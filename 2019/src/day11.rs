use super::intcode::intcode::{call_intcode, State};
use std::collections::HashMap;

#[aoc(day11, part1)]
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

#[aoc(day11, part2)]
pub fn solve_part2(input: &str) -> i128 {
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

    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    panels.insert("0:0".to_string(), 1);

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

        if curr_pos_x < min_x {
            min_x = curr_pos_x;
        }
        if curr_pos_x > max_x {
            max_x = curr_pos_x;
        }
        if curr_pos_y < min_y {
            min_y = curr_pos_y;
        }
        if curr_pos_y > max_y {
            max_y = curr_pos_y;
        }
    }

    for y in 0..(max_y - min_y + 1) {
        println!("");
        for x in 0..(max_x - min_y + 1) {
            print!(
                "{}",
                match panels.get(&format!("{}:{}", x + min_x, y + min_y)).copied() {
                    Some(1) => "#",
                    _ => " ",
                }
            )
        }
    }

    return panels.keys().len() as i128;
}

#[cfg(test)]
mod tests {
    use super::solve_part1 as part1;
    // use super::solve_part2 as part2;
}
