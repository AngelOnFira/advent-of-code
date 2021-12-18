use itertools::Itertools;
use regex::Regex;

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

fn test_explode(input: String) -> Option<String> {
    let mut bracket_count = 0;
    for (i, char) in input.chars().enumerate() {
        match char {
            '[' => bracket_count += 1,
            ']' => bracket_count -= 1,
            _ => (),
        }

        if bracket_count >= 5 && char == '[' {
            // read ahead to the next ']', and see if it matches the regex \[\d+,\d+\]
            let end_of_match = i + &input[i..].find(']').unwrap() + 1;
            let test_string = &input[i..end_of_match];
            let left_string = &input[..i];
            let right_string = &input[end_of_match..];

            let re = Regex::new(r"\[(\d+),(\d+)\]").unwrap();
            if re.is_match(test_string) {
                // Get the captures
                let captures = re.captures(test_string).unwrap();

                let left_num = captures[1].parse::<usize>().unwrap();
                let right_num = captures[2].parse::<usize>().unwrap();

                return Some(format!(
                    "{}0{}",
                    add_to_last_num_in_string(left_string.to_string(), left_num),
                    add_to_first_num_in_string(right_string.to_string(), right_num)
                ));
            }
        }
    }
    None
}

fn test_split(input: String) -> Option<String> {
    // Find the first place there is a number of 10 or more with regex
    let re = Regex::new(r"(\d{2,})").unwrap();
    if let Some(cap) = re.captures(&input) {
        // Get the position of the capture
        let pos = input.find(&cap[1]).unwrap();
        let num = cap[1].parse::<usize>().unwrap();
        let left = &input[..pos];
        let right = &input[pos + cap[1].len()..];

        let new_pair = format!(
            "[{},{}]",
            (num as f64 / 2.0).floor(),
            (num as f64 / 2.0).ceil()
        );

        return Some(format!("{}{}{}", left, new_pair, right));
    }
    None
}

fn add_snailfish_numbers(left: String, right: String) -> String {
    format!("[{},{}]", left, right)
}

fn add_to_last_num_in_string(input: String, num: usize) -> String {
    let mut pos = input.len() - 1;
    let mut start_num_pos = 0;
    let mut end_num_pos = 0;
    let mut on_num = false;

    while pos > 0 {
        if input.chars().nth(pos).unwrap().is_numeric() {
            if on_num {
                end_num_pos = pos;
            } else {
                start_num_pos = pos;
                end_num_pos = pos;
                on_num = true;
            }
        } else {
            if on_num {
                break;
            }
        }
        pos -= 1;
    }

    if start_num_pos != 0 {
        let first_half = &input[..end_num_pos];
        let second_half = &input[start_num_pos + 1..];
        let new_num = &input[end_num_pos..start_num_pos + 1]
            .parse::<usize>()
            .unwrap()
            + num;
        return format!("{}{}{}", first_half, new_num, second_half);
    }

    input
}

fn add_to_first_num_in_string(input: String, num: usize) -> String {
    let mut pos = 0;
    let mut start_num_pos = 0;
    let mut end_num_pos = 0;
    let mut on_num = false;

    while pos < input.len() {
        if input.chars().nth(pos).unwrap().is_numeric() {
            if on_num {
                end_num_pos = pos;
            } else {
                start_num_pos = pos;
                end_num_pos = pos;
                on_num = true;
            }
        } else {
            if on_num {
                break;
            }
        }
        pos += 1;
    }

    if start_num_pos != 0 {
        let first_half = &input[..start_num_pos];
        let second_half = &input[end_num_pos + 1..];
        let new_num = &input[start_num_pos..end_num_pos + 1]
            .parse::<usize>()
            .unwrap()
            + num;
        return format!("{}{}{}", first_half, new_num, second_half);
    }

    input
}

fn calculate_first_magnitude(input: String) -> Option<String> {
    // The magnitude of a pair is 3 times the magnitude of its left element plus
    // 2 times the magnitude of its right element. The magnitude of a regular
    // number is just that number.

    // For example, the magnitude of [9,1] is 3*9 + 2*1 = 29; the magnitude of [1,9]
    // is 3*1 + 2*9 = 21. Magnitude calculations are recursive: the magnitude of
    // [[9,1],[1,9]] is 3*29 + 2*21 = 129.

    // Find the first pair of numbers in the input string
    let re = Regex::new(r"\[(\d+,\d+)\]").unwrap();
    if let Some(cap) = re.captures(&input) {
        // Get the position of the capture
        let pos = input.find(&cap[1]).unwrap();
        let nums: Vec<u64> = cap[1]
            .split(",")
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        let left = &input[..pos - 1];
        let right = &input[pos + cap[1].len() + 1..];

        let new_num = nums[0] * 3 + nums[1] * 2;

        return Some(format!("{}{}{}", left, new_num, right));
    }
    None
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &Vec<String>) -> i32 {
    let final_snail = input
        .iter()
        .map(|s| s.to_string())
        .reduce(|acc, line| {
            let mut new_line = add_snailfish_numbers(acc.to_string(), line.to_string());
            let mut action = true;
            while action {
                action = false;
                if let Some(exploded_line) = test_explode(new_line.clone()) {
                    action = true;
                    new_line = exploded_line;
                    continue;
                }
                if let Some(split_line) = test_split(new_line.clone()) {
                    action = true;
                    new_line = split_line;
                    continue;
                }
            }
            new_line
        })
        .unwrap();

    let mut make_small = final_snail.clone();

    let mut action = true;
    while action {
        action = false;
        if let Some(magnitude) = calculate_first_magnitude(make_small.clone()) {
            action = true;
            make_small = magnitude;
        }
    }
    make_small.parse::<i32>().unwrap()
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &Vec<String>) -> i32 {
    input
        .iter()
        .map(|s| s.to_string())
        .permutations(2)
        .map(|perms| {
            let first = perms[0].clone();
            let second = perms[1].clone();

            let mut new_line = add_snailfish_numbers(first.to_string(), second.to_string());
            let mut action = true;
            while action {
                action = false;
                if let Some(exploded_line) = test_explode(new_line.clone()) {
                    action = true;
                    new_line = exploded_line;
                    continue;
                }
                if let Some(split_line) = test_split(new_line.clone()) {
                    action = true;
                    new_line = split_line;
                    continue;
                }
            }
            let mut action = true;
            while action {
                action = false;
                if let Some(magnitude) = calculate_first_magnitude(new_line.clone()) {
                    action = true;
                    new_line = magnitude;
                }
            }
            // Convert into i32
            new_line.parse::<i32>().unwrap()
        })
        .max()
        .unwrap()
}
