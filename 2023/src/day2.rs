use itertools::Itertools;

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> i32 {
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

    // Get this with regex
    // Find any that have more than 12 red, or 13 green, or 14 blue in one
    // place. Get the sums of the indexes of those lines.
    input
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            // Get colour counts using regex
            let red_vec = regex::Regex::new(r"(\d+) red").unwrap();
            let green_vec = regex::Regex::new(r"(\d+) green").unwrap();
            let blue_vec = regex::Regex::new(r"(\d+) blue").unwrap();

            let red_count = red_vec.captures_iter(line);
            let green_count = green_vec.captures_iter(line);
            let blue_count = blue_vec.captures_iter(line);

            // Find if any of the red numbers are greater than 12
            let red_count = red_count
                .map(|cap| cap[1].parse::<i32>().unwrap())
                .any(|x| x > 12);

            // Find if any of the green numbers are greater than 13
            let green_count = green_count
                .map(|cap| cap[1].parse::<i32>().unwrap())
                .any(|x| x > 13);

            // Find if any of the blue numbers are greater than 14
            let blue_count = blue_count
                .map(|cap| cap[1].parse::<i32>().unwrap())
                .any(|x| x > 14);

            // If any of the above are true, return true
            !(red_count || green_count || blue_count)
        })
        .map(|(index, _)| index + 1)
        .sum::<usize>() as i32
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> i32 {
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

    // Do the same as before, but this time find the max number of each
    input
        .lines()
        .enumerate()
        .map(|(_, line)| {
            // Get colour counts using regex
            let red_vec = regex::Regex::new(r"(\d+) red").unwrap();
            let green_vec = regex::Regex::new(r"(\d+) green").unwrap();
            let blue_vec = regex::Regex::new(r"(\d+) blue").unwrap();

            let red_count = red_vec.captures_iter(line);
            let green_count = green_vec.captures_iter(line);
            let blue_count = blue_vec.captures_iter(line);

            // Find the max
            let red_count = red_count
                .map(|cap| cap[1].parse::<i32>().unwrap())
                .max()
                .unwrap();

            // Find the max
            let green_count = green_count
                .map(|cap| cap[1].parse::<i32>().unwrap())
                .max()
                .unwrap();

            // Find the max
            let blue_count = blue_count
                .map(|cap| cap[1].parse::<i32>().unwrap())
                .max()
                .unwrap();

            // If any of the above are true, return true
            red_count * green_count * blue_count
        })
        .sum::<i32>() as i32
}

#[cfg(test)]
mod tests {
    // use super::solve_part1 as part1;
    // use super::solve_part2 as part2;
}
