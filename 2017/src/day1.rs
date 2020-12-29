#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut previous = input.chars().last().unwrap();
    input.chars().fold(0, |acc, character| {
        if character == previous {
            previous = character;
            return acc + character.to_digit(10).unwrap() as i32;
        }
        previous = character;
        acc
    })
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let circle: Vec<char> = input.chars().collect();

    (0..circle.len()).fold(0, |acc, i| {
        if circle[0] == circle[((i + circle.len() / 2) % circle.len())] {
            return acc + circle[i].to_digit(10).unwrap() as i32;
        }
        acc
    })
}
