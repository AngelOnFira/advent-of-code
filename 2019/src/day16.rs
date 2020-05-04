#[aoc(day16, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let pattern = [0, 1, 0, -1];

    let mut last_phase: Vec<i8> = input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i8)
        .collect::<Vec<i8>>();

    for _phase in 0..100 {
        let mut new_phase: Vec<i8> = Vec::new();
        for i in 0..last_phase.len() {
            let mut this_pattern = Vec::new();

            let mut pattern_counter = 0;
            let mut index_counter = 0;
            for _ in 0..last_phase.len() + 1 {
                this_pattern.push(pattern[pattern_counter % pattern.len()]);
                if index_counter == i {
                    index_counter = 0;
                    pattern_counter += 1;
                } else {
                    index_counter += 1;
                }
            }

            let mut total: i32 = 0;
            for j in 0..last_phase.len() {
                total += (last_phase[j] * this_pattern[j + 1]) as i32
            }
            new_phase.push((total.abs() % 10) as i8);
        }
        last_phase = new_phase;
    }

    let mut out = 0;

    for i in 0..8 {
        out += (last_phase[i] as i32) * 10i32.pow((7 - i) as u32);
    }

    out
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let _pattern = [0, 1, 0, -1];

    let base_phase: Vec<i8> = input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i8)
        .collect::<Vec<i8>>();

    let mut last_phase: Vec<i8> = Vec::new();
    for _ in 0..10_000 {
        last_phase.append(&mut base_phase.clone());
    }

    let mut message_loc = 0;
    for i in 0..7 {
        message_loc += (last_phase[i] as i32) * 10i32.pow((6 - i) as u32);
    }

    println!("{}", message_loc);

    for _phase in 0..100 {
        let mut new_phase: Vec<i8> = Vec::new();
        for _ in (message_loc as usize)..last_phase.len() {
            let mut total: i32 = 0;
            for j in (message_loc as usize)..last_phase.len() {
                total += last_phase[j] as i32
            }
            new_phase.push((total.abs() % 10) as i8);
        }
        last_phase = new_phase;
        println!("{:?}", last_phase);
    }

    let mut out = 0;

    for i in 0..8 {
        out += (last_phase[i + message_loc as usize] as i32) * 10i32.pow((7 - i) as u32);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::solve_part1 as part1;
    use super::solve_part2 as part2;

    #[test]
    fn sample1611() {
        assert_eq!(part1("69317163492948606335995924319873"), 52432133);
    }

    #[test]
    fn sample1621() {
        assert_eq!(part2("03036732577212944063491565474664"), 84462026);
    }
}
