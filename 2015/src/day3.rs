#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut count = 0;
    for character in input.chars() {
        match character {
            '(' => count += 1,
            ')' => count -= 1,
            _ => unreachable!(),
        };
    }
    count
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut count = 0;
    for (i, character) in input.chars().enumerate() {
        match character {
            '(' => count += 1,
            ')' => count -= 1,
            _ => unreachable!(),
        };
        if count < 0 {
            return (i + 1) as i32;
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    // use super::solve_part1 as part1;
    // use super::solve_part2 as part2;
}
