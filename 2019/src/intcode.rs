mod intcode {

#[derive(Clone)]
struct State {
    instruction_pointer: usize,
    instructions: Vec<i32>,
    input: Vec<i32>,
}

pub fn call_intcode(state: &mut State) -> i32 {
    loop {
        let opcode_int = state.instructions[state.instruction_pointer];

        let opcode_string = opcode_int.to_string();

        let opcode;
        let mut parameters: Vec<i32> = Vec::new();

        if opcode_string.len() > 1 && opcode_int != 99 {
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
        match opcode {
            1 => opcode_1(state, parameters),
            2 => opcode_2(state, parameters),
            3 => opcode_3(state),
            4 => return opcode_4(state, parameters),
            5 => opcode_5(state, parameters),
            6 => opcode_6(state, parameters),
            7 => opcode_7(state, parameters),
            8 => opcode_8(state, parameters),
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
    if mode == 1 {
        unreachable!();
    }

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
    let input = state.input.remove(0);
    set_position(
        state, 0, 1, input
    );
    state.instruction_pointer += 2;
}

fn opcode_4(state: &mut State, parameter_modes: Vec<i32>) -> i32 {
    let value = get_position(state, parameter_modes[0], 1);
    state.instruction_pointer += 2;
    value
}

fn opcode_5(state: &mut State, parameter_modes: Vec<i32>) {
    if get_position(state, parameter_modes[0], 1) != 0 {
        state.instruction_pointer = get_position(state, parameter_modes[1], 2) as usize;
    } else {
        state.instruction_pointer += 3;
    }
}

fn opcode_6(state: &mut State, parameter_modes: Vec<i32>) {
    if get_position(state, parameter_modes[0], 1) == 0 {
        state.instruction_pointer = get_position(state, parameter_modes[1], 2) as usize;
    } else {
        state.instruction_pointer += 3;
    }
}

fn opcode_7(state: &mut State, parameter_modes: Vec<i32>) {
    if get_position(state, parameter_modes[0], 1) < get_position(state, parameter_modes[1], 2) {
        set_position(state, parameter_modes[2], 3, 1);
    } else {
        set_position(state, parameter_modes[2], 3, 0);
    }
    state.instruction_pointer += 4;
}

fn opcode_8(state: &mut State, parameter_modes: Vec<i32>) {
    if get_position(state, parameter_modes[0], 1) == get_position(state, parameter_modes[1], 2) {
        set_position(state, parameter_modes[2], 3, 1);
    } else {
        set_position(state, parameter_modes[2], 3, 0);
    }
    state.instruction_pointer += 4;
}

}