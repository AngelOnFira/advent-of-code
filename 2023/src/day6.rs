#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> i64 {
    // Time:        60     94     78     82
    // Distance:   475   2138   1015   1650

    let races = vec![(60, 475), (94, 2138), (78, 1015), (82, 1650)];

    // Find how long we can hold the button for each race. Start

    races
        .iter()
        .map(|(t, d)| {
            let mut valid_count = 0;
            for i in 0..*t {
                if (t - i) * i >= *d {
                    valid_count += 1
                }
            }
            valid_count
        })
        .product()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> i64 {
    // Time:        60947882
    // Distance:   475213810151650

    let races: Vec<(i64, i64)> = vec![(60947882, 475213810151650)];

    // Find how long we can hold the button for each race. Start

    races
        .iter()
        .map(|(t, d)| {
            let mut valid_count = 0;
            for i in 0..*t {
                if (t - i) * i >= *d {
                    valid_count += 1
                }
            }
            valid_count
        })
        .product()
}

#[cfg(test)]
mod tests {
    // use super::solve_part1 as part1;
    // use super::solve_part2 as part2;
}
