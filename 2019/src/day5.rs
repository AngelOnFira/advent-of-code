struct State {
    instruction_pointer: usize,
    instructions: Vec<i32>,
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut state = State {
        instruction_pointer: 0,
        instructions: Default::default(),
    };

    state.instructions = input
        .split(",")
        .map(|input| input.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    loop {
        let opcode_int = state.instructions[state.instruction_pointer];

        let opcode_string = opcode_int.to_string();

        let opcode;
        let mut parameters: Vec<i32> = Vec::new();

        if opcode_string.len() > 1 {
            opcode = opcode_string[opcode_string.len() - 2..]
                .parse::<i32>()
                .unwrap();
            for exp in 0..3 {
                parameters.push((opcode_int % 10i32.pow(3 + exp)) / 10i32.pow(2 + exp));
            }
        } else {
            opcode = opcode_int;
            parameters = [0, 0, 0].to_vec();
        }
        //println!("{:?} {}", parameters, opcode);
        match opcode {
            1 => opcode_1(&mut state, parameters),
            2 => opcode_2(&mut state, parameters),
            3 => opcode_3(&mut state),
            4 => println!("{}", opcode_4(&mut state)),
            99 => return -1,
            _ => unreachable!(),
        }
    }
}

fn get_position(state: &mut State, mode: i32, offset: usize) -> i32 {
    if mode == 0 {
        return state.instructions[state.instructions[state.instruction_pointer + offset] as usize];
    } else if mode == 1 {
        return state.instructions[state.instruction_pointer + offset];
    }
    unreachable!();
}

fn set_position(state: &mut State, mode: i32, offset: usize, input: i32) {
    let location = state.instructions[state.instruction_pointer + offset] as usize;
    state.instructions[location] = input
}

fn opcode_1(state: &mut State, parameter_modes: Vec<i32>) {
    let value =
        get_position(state, parameter_modes[0], 1) + get_position(state, parameter_modes[1], 2);
    set_position(state, parameter_modes[2], 3, value);
    state.instruction_pointer += 4;
}

fn opcode_2(state: &mut State, parameter_modes: Vec<i32>) {
    let value =
        get_position(state, parameter_modes[0], 1) * get_position(state, parameter_modes[1], 2);
    set_position(state, parameter_modes[2], 3, value);
    state.instruction_pointer += 4;
}

fn opcode_3(state: &mut State) {
    set_position(
        state, 0, 1, 1, // This is hardcoded as part of the problem input
    );
    state.instruction_pointer += 2;
}

fn opcode_4(state: &mut State) -> i32 {
    let value = get_position(state, 0, 1);
    state.instruction_pointer += 2;
    value
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> i32 {
    12
}

#[cfg(test)]
mod tests {
    use super::solve_part1 as part1;
    //use super::solve_part2 as part2;

    #[test]
    fn sample511() {
        assert_eq!(part1("3,0,4,0,99"), 1);
    }

    #[test]
    fn sample512() {
        assert_eq!(part1("1101,100,-1,6,4,6,0"), 99);
    }

    #[test]
    fn sample513() {
        assert_eq!(part1("1101,100,-1,6,4,6,0"), 99);
    }
}
