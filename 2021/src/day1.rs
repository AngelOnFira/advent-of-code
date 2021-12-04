#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|input| input.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    input.windows(2).filter(|x| x[0] < x[1]).count() as i32
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    input
        .windows(3)
        .map(|x| x.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|x| x[0] < x[1])
        .count() as i32
}

#[cfg(test)]
mod tests {
    // use super::solve_part1 as part1;
    // use super::solve_part2 as part2;
}
