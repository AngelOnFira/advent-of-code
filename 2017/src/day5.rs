#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut instructions: Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut counter = 0;
    let mut pos = 0;
    loop {
        match instructions.clone().get(pos) {
            Some(x) => {
                instructions[pos] += 1;
                pos += *x as usize;
            }
            None => return counter,
        }
        counter += 1;
    }
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut instructions: Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut counter = 0;
    let mut pos = 0;
    loop {
        match instructions.clone().get(pos) {
            Some(x) => {
                if *x >= 3 {
                    instructions[pos] -= 1;
                } else {
                    instructions[pos] += 1;
                }
                pos += *x as usize;
            }
            None => return counter,
        }
        counter += 1;
    }
}
