use std::collections::HashSet;
//mod intcode::intcode;

#[aoc(day7, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut state = intcode::State {
        instruction_pointer: 0,
        instructions: Default::default(),
        input: Vec::new(),
    };

    state.instructions = input
        .split(",")
        .map(|input| input.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut max_output = 0;

    day1::solve_part1("test");

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

                                    current_result =
                                        intcode::call_intcode(&mut phase_states[i]);

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
