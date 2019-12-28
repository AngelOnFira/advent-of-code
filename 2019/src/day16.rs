#[aoc(day16, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let pattern = [0, 1, 0, -1];

    println!("{}", input);

    let mut last_phase: Vec<i8> = input.chars().map(|x| x as i8).collect::<Vec<i8>>();

    for phase in 0..1 {
        let mut new_phase: Vec<i8> = Vec::new();
        for i in 0..last_phase.len() {
            let mut this_pattern = Vec::new();

            let mut pattern_counter = 0;
            let mut index_counter = 1;
            for j in 0..last_phase.len() {
                if index_counter == i {
                    index_counter = 0;
                    pattern_counter += 1;
                } else {
                    index_counter += 1;
                }

                this_pattern.push(pattern[pattern_counter % pattern.len()]);
            }

            for j in 0..last_phase.len() {
                new_phase.push(last_phase[j] * this_pattern[j] % 10);
            }
        }
        println!("{:?}", new_phase);
        last_phase = new_phase;
    }

    let mut out = 0;

    for i in 0..8 {
        println!("{}", (last_phase[i] as i32));
        out += (last_phase[i] as i32) * 10i32.pow((7 - i) as u32);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::solve_part1 as part1;

    #[test]
    fn sample1611() {
        assert_eq!(part1("12345678"), 48226158);
    }
}
