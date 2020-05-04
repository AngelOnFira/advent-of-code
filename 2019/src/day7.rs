use std::collections::HashSet;

#[derive(Clone)]
struct State {
    instruction_pointer: usize,
    instructions: Vec<i32>,
    input: Vec<i32>,
}

//#[aoc(day7, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut state = State {
        instruction_pointer: 0,
        instructions: Default::default(),
        input: Vec::new(),
    };

    state.instructions = input
        .split(",")
        .map(|input| input.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut max_output = 0;

    for phase_a in 0..=4 {
        for phase_b in 0..=4 {
            for phase_c in 0..=4 {
                for phase_d in 0..=4 {
                    for phase_e in 0..=4 {
                        let mut unique: HashSet<i32> = HashSet::new();
                        unique.insert(phase_a);
                        unique.insert(phase_b);
                        unique.insert(phase_c);
                        unique.insert(phase_d);
                        unique.insert(phase_e);

                        if unique.len() == 5 {
                            let mut last_output = 0;
                            let phase_setting: Vec<i32> =
                                vec![phase_a, phase_b, phase_c, phase_d, phase_e];

                            for i in 0..5 {
                                let _input: Vec<i32> = vec![last_output, phase_setting[i]];

                                last_output = call_intcode(&mut state.clone());
                            }

                            if last_output > max_output {
                                max_output = last_output;
                            }
                        }
                    }
                }
            }
        }
    }

    return max_output;
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut state = State {
        instruction_pointer: 0,
        instructions: Default::default(),
        input: Vec::new(),
    };

    state.instructions = input
        .split(",")
        .map(|input| input.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut max_output = 0;

    for phase_a in 5..=9 {
        for phase_b in 5..=9 {
            for phase_c in 5..=9 {
                for phase_d in 5..=9 {
                    for phase_e in 5..=9 {
                        let mut unique: HashSet<i32> = HashSet::new();
                        unique.insert(phase_a);
                        unique.insert(phase_b);
                        unique.insert(phase_c);
                        unique.insert(phase_d);
                        unique.insert(phase_e);

                        if unique.len() == 5 {
                            let mut last_output = 0;
                            let mut current_result = 0;
                            let phase_setting: Vec<i32> =
                                vec![phase_a, phase_b, phase_c, phase_d, phase_e];

                            let mut phase_states: Vec<State> = vec![state.clone(); 5];

                            for i in 0..5 {
                                phase_states[i].input.push(phase_setting[i]);
                            }

                            while current_result != -1 {
                                last_output = current_result;
                                'feedback: for i in 0..5 {
                                    phase_states[i].input.push(current_result);

                                    current_result = call_intcode(&mut phase_states[i]);

                                    if current_result == -1 {
                                        break 'feedback;
                                    }
                                }
                            }

                            if last_output > max_output {
                                max_output = last_output;
                            }
                        }
                    }
                }
            }
        }
    }

    return max_output;
}

fn call_intcode(state: &mut State) -> i32 {
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
    set_position(state, 0, 1, input);
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

#[cfg(test)]
mod tests {
    use super::solve_part1 as part1;
    use super::solve_part2 as part2;

    #[test]
    fn sample711() {
        assert_eq!(
            part1("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
            43210
        );
    }

    #[test]
    fn sample712() {
        assert_eq!(part1("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"), 65210);
    }

    #[test]
    fn sample721() {
        assert_eq!(part2("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"), 139629729);
    }

    #[test]
    fn sample722() {
        assert_eq!(part2("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"), 18216);
    }
}
