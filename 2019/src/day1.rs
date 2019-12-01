#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> i32 {
    input
        .lines()
        .map(|input| {
            let mass: i32 = input.parse::<i32>().unwrap();
            mass / 3 - 2
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> i32 {
    input
        .lines()
        .map(|input| {
            let mut total = 0;
            let mut mass: i32 = input.parse::<i32>().unwrap();
            while mass >= 0 {
                mass = mass / 3 - 2;
                total += mass;
            }
            total -= mass;
            total
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::solve_part1 as part1;
    use super::solve_part2 as part2;

    #[test]
    fn sample1() {
        assert_eq!(part1("12"), 2);
        assert_eq!(part1("12\n14"), 4);
        assert_eq!(part1("1969"), 654);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2("1969"), 966);
        assert_eq!(part2("100756"), 50346);
    }
}
