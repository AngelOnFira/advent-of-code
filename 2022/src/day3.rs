use std::collections::{HashSet};

use itertools::Itertools;

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> i64 {
    // Read each line of characters, and make two sets. The first set is all the
    // unique characters in the first half of the line, and the second set is all
    // the unique characters in the second half of the line.
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            let first = first.chars().collect::<HashSet<_>>();
            let second = second.chars().collect::<HashSet<_>>();

            // Get the characters that are in both sets
            let intersection = first.intersection(&second);

            // Get the sum of the characters in the list where a = 1, b = 2, c = 3,
            // etc, and A = 27, B = 28, C = 29, etc.

            intersection
                .map(|c| match c {
                    'a' => 1,
                    'b' => 2,
                    'c' => 3,
                    'd' => 4,
                    'e' => 5,
                    'f' => 6,
                    'g' => 7,
                    'h' => 8,
                    'i' => 9,
                    'j' => 10,
                    'k' => 11,
                    'l' => 12,
                    'm' => 13,
                    'n' => 14,
                    'o' => 15,
                    'p' => 16,
                    'q' => 17,
                    'r' => 18,
                    's' => 19,
                    't' => 20,
                    'u' => 21,
                    'v' => 22,
                    'w' => 23,
                    'x' => 24,
                    'y' => 25,
                    'z' => 26,
                    'A' => 27,
                    'B' => 28,
                    'C' => 29,
                    'D' => 30,
                    'E' => 31,
                    'F' => 32,
                    'G' => 33,
                    'H' => 34,
                    'I' => 35,
                    'J' => 36,
                    'K' => 37,
                    'L' => 38,
                    'M' => 39,
                    'N' => 40,
                    'O' => 41,
                    'P' => 42,
                    'Q' => 43,
                    'R' => 44,
                    'S' => 45,
                    'T' => 46,
                    'U' => 47,
                    'V' => 48,
                    'W' => 49,
                    'X' => 50,
                    'Y' => 51,
                    'Z' => 52,
                    _ => 0,
                })
                .sum::<i64>()
        })
        .sum()
}

fn char_to_num(c: char) -> i64 {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => 0,
    }
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> i64 {
    // Read each line of characters, and make two sets. The first set is all the
    // unique characters in the first half of the line, and the second set is all
    // the unique characters in the second half of the line.
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut line| {
            // Find the only similar character between the three lines
            let first = line
                .next()
                .unwrap()
                .chars()
                .map(char_to_num)
                .collect::<HashSet<_>>();
            let second = line
                .next()
                .unwrap()
                .chars()
                .map(char_to_num)
                .collect::<HashSet<_>>();
            let third = line
                .next()
                .unwrap()
                .chars()
                .map(char_to_num)
                .collect::<HashSet<_>>();

            let intersection = first
                .intersection(&second)
                .map(|x| x.to_owned())
                .collect::<HashSet<i64>>();
            let intersection = intersection
                .intersection(&third)
                .map(|x| x.to_owned())
                .collect::<HashSet<i64>>();

            // Get the score of this last character
            intersection.iter().map(|x| *x).sum::<i64>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    // use super::solve_part1 as part1;
    // use super::solve_part2 as part2;
}
