use regex::Regex;

pub struct Instruction {
    direction: Direction,
    distance: usize,
}

pub enum Direction {
    Forward,
    Down,
    Up,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    // forward 5
    // down 5
    // forward 8
    // up 3
    // down 8
    // forward 2
    
    input
        .lines()
        .map(|line| {
            let re = Regex::new(r"([a-z]+) (\d+)").unwrap();
            let caps = re.captures(line).unwrap();
            let direction = match &caps[1] {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => panic!("Unknown direction"),
            };
            let distance = caps[2].parse::<usize>().unwrap();
            Instruction {
                direction,
                distance,
            }
        })
        .collect::<Vec<Instruction>>()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Instruction]) -> i32 {
    // forward X increases the horizontal position by X units.
    // down X increases the depth by X units.
    // up X decreases the depth by X units.

    let mut horizontal_position = 0;
    let mut depth = 0;

    for instruction in input {
        match instruction.direction {
            Direction::Forward => horizontal_position += instruction.distance,
            Direction::Down => depth += instruction.distance,
            Direction::Up => depth -= instruction.distance,
        }
    }

    (horizontal_position * depth) as i32
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Instruction]) -> i32 {
    // down X increases your aim by X units.
    // up X decreases your aim by X units.
    // forward X does two things:
    //     It increases your horizontal position by X units.
    //     It increases your depth by your aim multiplied by X.

    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for instruction in input {
        match instruction.direction {
            Direction::Forward => {
                horizontal_position += instruction.distance;
                depth += aim * instruction.distance;
            }
            Direction::Down => aim += instruction.distance,
            Direction::Up => aim -= instruction.distance,
        }
    }

    (horizontal_position * depth) as i32
}
