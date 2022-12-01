#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> i32 {
    // Read and add up each line and add up numbers until there is a blank line.
    // Return the max
    let mut curr_counter = 0;
    let mut max_counter = 0;
    for line in input.lines() {
        if line == "" {
            if curr_counter > max_counter {
                max_counter = curr_counter;
            }
            dbg!(curr_counter);
            curr_counter = 0;
        } else {
            curr_counter += line.parse::<i32>().unwrap();
        }
    }
    max_counter
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> i32 {
    // Do the same as above but sum the top 3
    let mut curr_counter = 0;
    let mut sums = Vec::new();
    for line in input.lines() {
        if line == "" {
            sums.push(curr_counter);
            curr_counter = 0;
        } else {
            curr_counter += line.parse::<i32>().unwrap();
        }
    }
    sums.sort();
    
    
    // Add up the top 3
    sums[sums.len() - 1] + sums[sums.len() - 2] + sums[sums.len() - 3]

}

#[cfg(test)]
mod tests {
    // use super::solve_part1 as part1;
    // use super::solve_part2 as part2;
}
