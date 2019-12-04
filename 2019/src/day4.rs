#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let a: i32 = input.split("-").collect::<Vec<&str>>()[0 as usize]
        .parse::<i32>()
        .unwrap();
    let b: i32 = input.split("-").collect::<Vec<&str>>()[1 as usize]
        .parse::<i32>()
        .unwrap();

    let mut total = 0;

    for i in a..=b {
        let mut good = true;
        let mut double = false;
        let mut last_digit = 0;
        let mut check = i.clone();
        let mut num_len = (i as f32).log(10f32) as u32;
        for exp in 0..=num_len {
            let this_digit = check / 10i32.pow(num_len - exp);
            //println!("{}", this_digit);
            if !(this_digit >= last_digit) {
                good = false;
            }
            if this_digit == last_digit {
                double = true;
            }
            last_digit = this_digit;
            check = check % 10i32.pow(num_len - exp);
        }
        if good && double {
            total += 1;
        }
    }
    total
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let a: i32 = input.split("-").collect::<Vec<&str>>()[0 as usize]
        .parse::<i32>()
        .unwrap();
    let b: i32 = input.split("-").collect::<Vec<&str>>()[1 as usize]
        .parse::<i32>()
        .unwrap();

    let mut total = 0;

    for i in a..=b {
        let mut good = true;
        let mut double = 0;
        let mut same_count = 0;
        let mut last_digit = 0;
        let mut check = i.clone();
        let mut num_len = (i as f32).log(10f32) as u32;
        for exp in 0..=num_len {
            let this_digit = check / 10i32.pow(num_len - exp);
            //println!("{}", this_digit);
            if !(this_digit >= last_digit) {
                good = false;
            }
            if this_digit == last_digit {
                same_count += 1;
                if same_count == 1 {
                double += 1;
                }
                else if same_count == 2 {
                    double -= 1;
                }
            }
            else {
                same_count = 0;
            }
            last_digit = this_digit;
            check = check % 10i32.pow(num_len - exp);
        }
        if good && double > 0 {
            total += 1;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::solve_part1 as part1;
    use super::solve_part2 as part2;

    #[test]
    fn sample311() {
        assert_eq!(part1("122-600"), 1);
    }
}
