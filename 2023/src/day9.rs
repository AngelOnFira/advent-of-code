#[aoc(day9, part1)]
pub fn solve_part1(input: &str) -> i64 {
    //     0 3 6 9 12 15
    // 1 3 6 10 15 21
    // 10 13 16 21 30 45
    // Read the numbers from each line
    // Store them in a vector
    let lines: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|num| num.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    // Find the difference in each number in each line. If this new vec of
    // numbers isn't all zeros, repeat.
    lines
        .into_iter()
        .map(|line| {
            let mut derivatives = vec![line];
            loop {
                // Read the last line of derivatives. If the numbers aren't all
                // zero, then get the differece of each number in the line and
                // collect that into a new line of derivatives.
                let last_line = derivatives.last().unwrap();
                let mut new_line = vec![];
                for i in 0..last_line.len() - 1 {
                    let diff = last_line[i + 1] - last_line[i];
                    new_line.push(diff);
                }
                derivatives.push(new_line.clone());

                // If the new line is all zeros, break
                if new_line.iter().all(|&x| x == 0) {
                    break;
                }
            }

            dbg!(&derivatives);

            // Now, we can get the next number of the first derivative. This is done
            // by adding the last number of the second last derivative to the last
            // number of the one above it, then repeating to the top.
            derivatives.iter().rev().fold(0, |acc, line| {
                let next = acc + line[line.len() - 1];
                next
            })
        })
        .sum()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &str) -> i64 {
    //     0 3 6 9 12 15
    // 1 3 6 10 15 21
    // 10 13 16 21 30 45
    // Read the numbers from each line
    // Store them in a vector
    let lines: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|num| num.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    // Find the difference in each number in each line. If this new vec of
    // numbers isn't all zeros, repeat.
    lines
        .into_iter()
        .map(|line| {
            let mut derivatives = vec![line];
            loop {
                // Read the last line of derivatives. If the numbers aren't all
                // zero, then get the differece of each number in the line and
                // collect that into a new line of derivatives.
                let last_line = derivatives.last().unwrap();
                let mut new_line = vec![];
                for i in 0..last_line.len() - 1 {
                    let diff = last_line[i + 1] - last_line[i];
                    new_line.push(diff);
                }
                derivatives.push(new_line.clone());

                // If the new line is all zeros, break
                if new_line.iter().all(|&x| x == 0) {
                    break;
                }
            }

            // dbg!(&derivatives);

            // Now, we can get the next number of the first derivative. This is done
            // by adding the last number of the second last derivative to the last
            // number of the one above it, then repeating to the top.
            derivatives.iter().rev().fold(0, |acc, line| {
                let next = line[0] - acc;
                next
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    // use super::solve_part1 as part1;
    // use super::solve_part2 as part2;
}
