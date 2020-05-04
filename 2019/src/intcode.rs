pub mod intcode {

    use std::collections::HashMap;
    #[derive(Clone)]
    pub struct State {
        pub instruction_pointer: usize,
        pub relative_pointer: usize,
        pub instructions: Vec<i64>,
        pub input: Vec<i64>,
        pub output: Vec<i64>,
        pub memory: HashMap<usize, i64>,
    }

    pub fn call_intcode(state: &mut State) -> i64 {
        loop {
            let opcode_int = state.instructions[state.instruction_pointer];

            let opcode_string = opcode_int.to_string();

            let opcode;
            let mut parameters: Vec<i64> = Vec::new();

            if opcode_string.len() > 1 && opcode_int != 99 {
                opcode = opcode_string[opcode_string.len() - 2..]
                    .parse::<i64>()
                    .unwrap();
                for exp in 0..3 {
                    parameters.push((opcode_int % 10i64.pow(3 + exp)) / 10i64.pow(2 + exp));
                }
            } else {
                opcode = opcode_int;
                parameters = [0, 0, 0].to_vec();
            }

            match opcode {
                1 => opcode_1(state, parameters),
                2 => opcode_2(state, parameters),
                3 => opcode_3(state),
                4 => {
                    let o = opcode_4(state, parameters);
                    state.output.push(o);
                }
                5 => opcode_5(state, parameters),
                6 => opcode_6(state, parameters),
                7 => opcode_7(state, parameters),
                8 => opcode_8(state, parameters),
                9 => opcode_9(state, parameters),
                99 => return -1,
                _ => unreachable!(),
            }
        }
    }

    fn get_position(state: &mut State, mode: i64, offset: usize) -> i64 {
        let location;
        // Get from position at location
        if mode == 0 {
            location = state.instructions[state.instruction_pointer + offset] as usize
        // Get from absolute position
        } else if mode == 1 {
            location = state.instruction_pointer + offset as usize;
        // Get from relative position
        } else if mode == 2 {
            location = (state.relative_pointer as i64 + state.instructions[state.instruction_pointer + offset]) as usize;
        } else {
            unimplemented!()
        }

        if location > state.instructions.len() {
            match state.memory.get(&location).copied() {
                Some(value) => return value,
                None => return 0,
            }
        } else {
            return state.instructions[location];
        }

    }

    fn set_position(state: &mut State, mode: i64, offset: usize, input: i64) {
        if mode == 1 {
            unreachable!();
        }

        let location = state.instructions[state.instruction_pointer + offset] as usize;

        if location > state.instructions.len() {
            state.memory.insert(location, input);
        }
        else {
            state.instructions[location] = input;
        }
    }

    fn opcode_1(state: &mut State, parameter_modes: Vec<i64>) {
        let value =
            get_position(state, parameter_modes[0], 1) + get_position(state, parameter_modes[1], 2);
        set_position(state, parameter_modes[2], 3, value);
        state.instruction_pointer += 4;
    }

    fn opcode_2(state: &mut State, parameter_modes: Vec<i64>) {
        let value =
            get_position(state, parameter_modes[0], 1) * get_position(state, parameter_modes[1], 2);
        set_position(state, parameter_modes[2], 3, value);
        state.instruction_pointer += 4;
    }

    fn opcode_3(state: &mut State) {
        if state.input.len() == 0 {
            panic!("Trying to get input but there is none!");
        }
        let input = state.input.remove(0);
        set_position(state, 0, 1, input);
        state.instruction_pointer += 2;
    }

    fn opcode_4(state: &mut State, parameter_modes: Vec<i64>) -> i64 {
        let value = get_position(state, parameter_modes[0], 1);
        state.instruction_pointer += 2;
        value
    }

    fn opcode_5(state: &mut State, parameter_modes: Vec<i64>) {
        if get_position(state, parameter_modes[0], 1) != 0 {
            state.instruction_pointer = get_position(state, parameter_modes[1], 2) as usize;
        } else {
            state.instruction_pointer += 3;
        }
    }

    fn opcode_6(state: &mut State, parameter_modes: Vec<i64>) {
        if get_position(state, parameter_modes[0], 1) == 0 {
            state.instruction_pointer = get_position(state, parameter_modes[1], 2) as usize;
        } else {
            state.instruction_pointer += 3;
        }
    }

    fn opcode_7(state: &mut State, parameter_modes: Vec<i64>) {
        if get_position(state, parameter_modes[0], 1) < get_position(state, parameter_modes[1], 2) {
            set_position(state, parameter_modes[2], 3, 1);
        } else {
            set_position(state, parameter_modes[2], 3, 0);
        }
        state.instruction_pointer += 4;
    }

    fn opcode_8(state: &mut State, parameter_modes: Vec<i64>) {
        if get_position(state, parameter_modes[0], 1) == get_position(state, parameter_modes[1], 2)
        {
            set_position(state, parameter_modes[2], 3, 1);
        } else {
            set_position(state, parameter_modes[2], 3, 0);
        }
        state.instruction_pointer += 4;
    }

    fn opcode_9(state: &mut State, parameter_modes: Vec<i64>) {
        state.relative_pointer += get_position(state, parameter_modes[0], 1) as usize;
        state.instruction_pointer += 2;
    }
}
