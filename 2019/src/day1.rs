#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mass: i32 = input.parse::<i32>().unwrap();
    mass / 3 - 2
}

#[cfg(test)]
mod tests {
    use super::{solve_part1 as part1};

    // (()) and ()() both result in floor 0.
    #[test]
    fn sample1() {
        assert_eq!(part1("12"), 2);
        assert_eq!(part1("14"), 2);
    }
}
