use std::collections::HashSet;

use itertools::Itertools;

#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> i64 {
    // 21-81,20-96
    // 14-80,14-79
    // 87-89,7-88

    input
        .lines()
        .map(|x| {
            let first = x
                .split(',')
                .next()
                .unwrap()
                .split('-')
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap();
            let second = x
                .split(',')
                .next()
                .unwrap()
                .split('-')
                .last()
                .unwrap()
                .parse::<i64>()
                .unwrap();
            let third = x
                .split(',')
                .last()
                .unwrap()
                .split('-')
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap();
            let fourth = x
                .split(',')
                .last()
                .unwrap()
                .split('-')
                .last()
                .unwrap()
                .parse::<i64>()
                .unwrap();

            // Find if either range fits into the other
            if first >= third && second <= fourth {
                true
            } else if third >= first && fourth <= second {
                true
            } else {
                false
            }
        })
        .filter(|x| *x)
        .count() as i64
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> i64 {
    // 21-81,20-96
    // 14-80,14-79
    // 87-89,7-88

    input
        .lines()
        .map(|x| {
            let first = x
                .split(',')
                .next()
                .unwrap()
                .split('-')
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap();
            let second = x
                .split(',')
                .next()
                .unwrap()
                .split('-')
                .last()
                .unwrap()
                .parse::<i64>()
                .unwrap();
            let third = x
                .split(',')
                .last()
                .unwrap()
                .split('-')
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap();
            let fourth = x
                .split(',')
                .last()
                .unwrap()
                .split('-')
                .last()
                .unwrap()
                .parse::<i64>()
                .unwrap();

            // Same as part one, see if the ranges overlap at all
            (first..=second)
                .collect::<HashSet<_>>()
                .intersection(&(third..=fourth).collect::<HashSet<_>>())
                .count()
                > 0
        })
        .filter(|x| *x)
        .count() as i64
}

#[cfg(test)]
mod tests {
    // use super::solve_part1 as part1;
    // use super::solve_part2 as part2;
}
