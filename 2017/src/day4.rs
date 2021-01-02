use itertools::Itertools;
use std::collections::HashSet;

#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> i32 {
    input
        .lines()
        .filter(|line| {
            let mut set = HashSet::new();
            for word in line.split_whitespace() {
                if !set.insert(word) {
                    return false;
                }
            }
            return true;
        })
        .count() as i32
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> i32 {
    input
        .lines()
        .filter(|line| {
            let mut set = HashSet::new();
            for word in line.split_whitespace() {
                if !set.insert(word.chars().sorted().collect::<String>()) {
                    return false;
                }
            }
            return true;
        })
        .count() as i32
}
