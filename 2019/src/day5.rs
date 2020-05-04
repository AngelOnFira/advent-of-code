use super::intcode::intcode::{call_intcode, State};

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> i64 {
    let mut state = State {
        instruction_pointer: 0,
        relative_pointer: 0,
        instructions: Default::default(),
        input: Vec::new(),
        output: Vec::new(),
        memory: Default::default(),
    };

    state.input.push(1);

    state.instructions = input
        .split(",")
        .map(|input| input.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    call_intcode(&mut state);

    // Print the diagnostics
    // println!("{:?}", state.output);

    state.output.last().copied().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> i64 {
    let mut state = State {
        instruction_pointer: 0,
        relative_pointer: 0,
        instructions: Default::default(),
        input: Vec::new(),
        output: Vec::new(),
        memory: Default::default(),
    };

    state.instructions = input
        .split(",")
        .map(|input| input.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    state.input.push(5);

    call_intcode(&mut state);

    state.output.last().copied().unwrap()
}

#[cfg(test)]
mod tests {
    use super::solve_part1 as part1;
    use super::solve_part2 as part2;

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

    #[test]
    fn sample521() {
        assert_eq!(part2("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"), 12321);
    }
}
