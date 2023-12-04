use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> i32 {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11

    // Parse the numbers on the left of the | into a "winning" vec, and the ones
    // on the right to a "we got" vec
    input
        .lines()
        .map(|line| {
            let (winning, got) = line.split(" | ").collect_tuple().unwrap();
            let winning = winning.split(": ").nth(1).unwrap();
            dbg!(&winning);
            // Replace any double spaces with single spaces
            let winning = winning.replace("  ", " ");
            let winning = winning.strip_prefix(" ").unwrap_or(&winning);
            let got = got.replace("  ", " ");
            let got = got.strip_prefix(" ").unwrap_or(&got);
            dbg!(&got);
            let winning = winning
                .split(" ")
                .map(|n| n.parse::<i32>().unwrap_or(0))
                .collect::<Vec<i32>>();
            let got = got
                .split(" ")
                .map(|n| n.parse::<i32>().unwrap_or(0))
                .collect::<Vec<i32>>();

            // Find the number of matching numbers between the two vecs. This is
            // then worth 1/2/4/8/16 points.
            let mut points = 0;

            for num in &winning {
                if got.contains(num) {
                    dbg!(num);
                    points += 1;
                }
            }

            // If points is 0, then we didn't get any numbers, so we get 0 points.
            // Otherwise, we get 2 ^ points points.
            if points == 0 {
                return 0;
            }

            dbg!(2i32.pow(points as u32 - 1));

            // Return 2 ^ points
            2i32.pow(points as u32 - 1)
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> i32 {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11

    let mut instances_count = HashMap::new();

    // Parse the numbers on the left of the | into a "winning" vec, and the ones
    // on the right to a "we got" vec
    for (i, line) in input.lines().enumerate() {
        let (winning, got) = line.split(" | ").collect_tuple().unwrap();
        let winning = winning.split(": ").nth(1).unwrap();
        // Replace any double spaces with single spaces
        let winning = winning.replace("  ", " ");
        let winning = winning.strip_prefix(" ").unwrap_or(&winning);
        let got = got.replace("  ", " ");
        let got = got.strip_prefix(" ").unwrap_or(&got);
        let winning = winning
            .split(" ")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let got = got
            .split(" ")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        // Find the number of matching numbers between the two vecs. This is
        // then worth 1/2/4/8/16 points.
        let mut points = 0;

        for num in &winning {
            if got.contains(num) {
                points += 1;
            }
        }

        // Add the score to the instances_count for the next cards
        let curr_cards = instances_count.entry(i).or_insert(1).to_owned();
        for count in i + 1..i + 1 + points {
            *instances_count.entry(count).or_insert(1) += curr_cards;
        }
    }

    // Get the sum of all the values in the hashmap
    instances_count.values().sum()
}

#[cfg(test)]
mod tests {
    // use super::solve_part1 as part1;
    // use super::solve_part2 as part2;
}
