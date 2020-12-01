#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let nums: Vec<i32> = input
        .lines()
        .map(|input| input.parse::<i32>().unwrap())
        .collect();

    for i in nums.clone().iter() {
        for j in nums.clone().iter() {
            if i + j == 2020 {
                return i * j;
            }
        }
    }
    unreachable!();
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let nums: Vec<i32> = input
        .lines()
        .map(|input| input.parse::<i32>().unwrap())
        .collect();

    for i in nums.clone().iter() {
        for j in nums.clone().iter() {
            for k in nums.clone().iter() {
                if i + j + k == 2020 {
                    return i * j * k;
                }
            }
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    // use super::solve_part1 as part1;
    // use super::solve_part2 as part2;
}
