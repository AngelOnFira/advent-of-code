use itertools::Itertools;

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> i32 {
    input.lines().fold(0, |acc, line| {
        let line_ints: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        acc + line_ints.iter().max().unwrap() - line_ints.iter().min().unwrap()
    })
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> i32 {
    input.lines().fold(0, |acc, line| {
        for pair in line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
            .into_iter()
            .combinations(2)
        {
            if pair[0] % pair[1] == 0 {
                return acc + pair[0] / pair[1];
            }

            if pair[1] % pair[0] == 0 {
                return acc + pair[1] / pair[0];
            }
        }
        acc
    })
}
