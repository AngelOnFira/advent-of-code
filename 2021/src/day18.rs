use num_traits::Float;
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
            let mut left_string = &input[..i];
            let mut right_string = &input[end_of_match..];
            dbg!(test_string);

            let re = Regex::new(r"\[(\d+),(\d+)\]").unwrap();
            if re.is_match(test_string) {
                dbg!("insidee");
                // Get the captures
                let captures = re.captures(test_string).unwrap();

                let left_num = captures[1].parse::<usize>().unwrap();
                let right_num = captures[2].parse::<usize>().unwrap();

                let mut left_num_size = 0;
                let mut right_num_size = 0;

                // Get the last number of the left string
                let left_re = Regex::new(r"(\d+)(?!.*\d)").unwrap();

                if let Some(left_good) = left_re.captures(left_string) {
                    let this_num = left_good[1].parse::<usize>().unwrap();
                    let new_num = this_num + left_num;
                    // Replace in the string using regex
                    let left_string = left_re.replace(left_string, &new_num.to_string());
                };

                // Get the first number of the right string
                let right_re = Regex::new(r"(\d+)").unwrap();

                if let Some(right_good) = right_re.captures(right_string) {
                    let this_num = right_good[1].parse::<usize>().unwrap();
                    let new_num = this_num + right_num;
                    // Replace in the string using regex
                    let right_string = right_re.replace(right_string, &new_num.to_string());
                };

                // let right_re = Regex::new(r"^],(\d+)\]").unwrap();
                // let right_regular = match right_re.captures(&input[end_of_match..]) {
                //     Some(right_good) => {
                //         let found_chars = right_good[1].chars().collect::<String>();
                //         right_num_size = found_chars.len();
                //         right_num + found_chars.parse::<usize>().unwrap()
                //     }
                //     None => 0,
                // };

                // let new_pair = format!("[{},{}]", left_regular, right_regular);

                return Some(format!("{}0{}", left_string, right_string));
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
        let right = &input[pos + 2..];

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

#[aoc(day18, part1)]
pub fn solve_part1(input: &Vec<String>) -> i32 {
    let final_snail = input
        .iter()
        .map(|s| s.to_string())
        .reduce(|acc, line| {
            let mut new_line = add_snailfish_numbers(acc.to_string(), line.to_string());
            dbg!(new_line.clone());
            let mut action = true;
            let mut x = 0;
            while action {
                x += 1;
                if x > 10 {
                    break;
                }
                dbg!(new_line.clone());
                action = false;
                if let Some(exploded_line) = test_explode(new_line.clone()) {
                    dbg!("exploded");
                    action = true;
                    new_line = exploded_line;
                    continue;
                }
                if let Some(split_line) = test_split(new_line.clone()) {
                    dbg!("split");
                    action = true;
                    new_line = split_line;
                    continue;
                }
            }
            new_line
        })
        .unwrap();
    // dbg!(final_snail);
    3
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &Vec<String>) -> i32 {
    3
}
