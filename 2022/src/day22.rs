use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

use itertools::Itertools;
use regex::Regex;

type InputType = (HashMap<(i32, i32), Kind>, Vec<Instruction>);

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Kind {
    Wall,
    Floor,
    None,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Instruction {
    Distance(i32),
    Left,
    Right,
}

#[aoc_generator(day22)]
fn parse_input_day22(input: &str) -> (HashMap<(i32, i32), Kind>, Vec<Instruction>) {
    let parts = input.split("\n\n").collect_vec();

    // For part one, break it down into a map
    let map: HashMap<(i32, i32), Kind> = parts[0]
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    (
                        (x as i32, y as i32),
                        match c {
                            '#' => Kind::Wall,
                            '.' => Kind::Floor,
                            _ => Kind::None,
                        },
                    )
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let mut instructions = Vec::new();

    let mut num_build = String::new();
    for char in parts[1].chars() {
        match char {
            '0'..='9' => num_build.push(char),
            'L' | 'R' => {
                instructions.push(Instruction::Distance(num_build.parse::<i32>().unwrap()));
                num_build = "".to_string();
                if char == 'L' {
                    instructions.push(Instruction::Left)
                } else {
                    instructions.push(Instruction::Right)
                }
            }
            _ => unreachable!(),
        }
    }

    (map, instructions)
}

#[aoc(day22, part1)]
pub fn solve_part1(input: &InputType) -> i128 {
    let (map, instructions) = input;
    // Set the start position to the top left most open tile of the map
    let mut pos = map
        .iter()
        .filter(|((x, y), v)| *y == 0 && **v == Kind::Floor)
        .map(|((x, y), v)| (y, (x, y)))
        .min()
        .unwrap()
        .1;

    let mut pos = (*pos.0, *pos.1);

    // Facing is 0 for right (>), 1 for down (v), 2 for left (<), and 3 for up (^)
    let dirs = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    // Iterate over each instruction and print it out
    let mut facing = 0;
    for instruction in instructions {
        // dbg!(instruction.clone());
        dbg!(pos.clone());
        match instruction {
            Instruction::Distance(dist) => {
                let dist = *dist;
                for _ in 0..dist {
                    let mut original_pos = pos.clone();
                    let mut test_pos = pos.clone();

                    pos = (pos.0 + dirs[facing].0, pos.1 + dirs[facing].1);
                    match map.get(&pos).unwrap_or(&Kind::None) {
                        Kind::Wall => {
                            // If we've hit a wall, go back one
                            pos = (pos.0 - dirs[facing].0, pos.1 - dirs[facing].1);
                            break;
                        }
                        Kind::None => {
                            // If we've gone out of bounds, wrap around to the other
                            // wall from here. This means we need to go in the opposite
                            // direction to the last thing we hit in the other direction
                            // before the None kind. If it's a wall, we instead stay
                            // where we are. If not, we move there and continue.

                            let mut last_post = pos.clone();
                            while map.get(&test_pos).unwrap_or(&Kind::None) != &Kind::None {
                                last_post = test_pos.clone();
                                test_pos = (
                                    test_pos.0 + dirs[(facing + 2) % 4].0,
                                    test_pos.1 + dirs[(facing + 2) % 4].1,
                                );
                                println!(
                                    "{:?} {:?} {:?}",
                                    test_pos,
                                    last_post,
                                    map.get(&last_post).unwrap_or(&Kind::None)
                                );
                            }
                            match map.get(&last_post).unwrap_or(&Kind::None) {
                                Kind::Wall => {
                                    pos = original_pos;
                                    break;
                                }
                                Kind::Floor => {
                                    pos = last_post;
                                }
                                Kind::None => {
                                    unreachable!()
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            Instruction::Left => {
                facing = (facing + 3) % 4;
            }
            Instruction::Right => {
                facing = (facing + 1) % 4;
            }
        }
    }

    // To finish providing the password to this strange input device, you need
    // to determine numbers for your final row, column, and facing as your final
    // position appears from the perspective of the original map. Rows start
    // from 1 at the top and count downward; columns start from 1 at the left
    // and count rightward. (In the above example, row 1, column 1 refers to the
    // empty space with no tile on it in the top-left corner.) Facing is 0 for
    // right (>), 1 for down (v), 2 for left (<), and 3 for up (^). The final
    // password is the sum of 1000 times the row, 4 times the column, and the
    // facing.

    let row = pos.1 + 1;
    let col = pos.0 + 1;
    let facing = facing;

    (1000 * row + 4 * col + facing as i32) as i128
}

#[aoc(day22, part2)]
pub fn solve_part2(input: &InputType) -> i128 {
    0
}
