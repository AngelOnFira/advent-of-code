use super::intcode::intcode::{call_intcode, State};

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> i128 {
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
        .map(|input| input.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();

    state.instructions[1] = 12;
    state.instructions[2] = 2;

    call_intcode(&mut state, false);

    state.instructions[0]
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> i32 {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut number = input
                .split(",")
                .map(|input| input.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            number[1] = noun;
            number[2] = verb;
            let mut index = 0;
            while index + 4 < number.len() {
                let opcode = number[index];
                let read_loc1 = number[index + 1] as usize;
                let read_loc2 = number[index + 2] as usize;
                let out_loc = number[index + 3] as usize;
                match opcode {
                    1 => number[out_loc] = number[read_loc1] + number[read_loc2],
                    2 => number[out_loc] = number[read_loc1] * number[read_loc2],
                    99 => index = number.len(),
                    _ => unreachable!(),
                }
                index += 4;
            }
            if number[0] == 19690720 {
                println!("{} {}", noun, verb);
                return 100 * noun + verb;
            }
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::solve_part1 as part1;
    use super::solve_part2 as part2;

    #[test]
    fn sample1() {
        assert_eq!(part1("1,0,0,0,99"), 2);
    }

    #[test]
    fn sample2() {
        assert_eq!(part1("1,1,1,4,99,5,6,0,99"), 30);
    }

    #[test]
    fn sample3() {
        assert_eq!(part1("1,9,10,3,2,3,11,0,99,30,40,50"), 3500);
    }

    #[test]
    fn sample4() {
        assert_eq!(part2("1, 0, 0, 0, 99"), 1202);
    }
}
