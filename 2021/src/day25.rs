use std::collections::HashMap;

use regex::Regex;

pub struct Instruction {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Place {
    East,
    South,
    Empty,
}

const xlen: usize = 8;
const ylen: usize = 7;

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> [[Place; xlen]; ylen] {
    // v...>>.vv>
    // .vv>>.vv..
    // >>.>v>...v
    // >>v>>.>.v.
    // v>v.vv.v..
    // >.>>..v...
    // .vv..>.>v.
    // v.v..>>v.v
    // ....v..v.>

    let mut map = [[Place::Empty; xlen]; ylen];

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            map[y][x] = match c {
                '>' => Place::East,
                'v' => Place::South,
                '.' => Place::Empty,
                _ => panic!("Unknown character {}", c),
            };
        });
    });

    map
}

#[aoc(day25, part1)]
pub fn solve_part1(input: &[[Place; xlen]; ylen]) -> i32 {
    let mut old_map = input.clone();

    let mut i = 0;
    loop {
        i += 1;
        let mut new_map = old_map.clone();

        // Move all the east facing first
        for y in 0..ylen {
            for x in 0..xlen {
                if let Place::East = new_map[y][x] {
                    let mut new_x = x + 1;
                    let mut new_y = y;

                    if new_x == xlen {
                        new_x = 0;
                    }

                    // If this space is empty, move to it
                    if new_map[new_y][new_x] == Place::Empty {
                        new_map[new_y][new_x] = Place::East;
                        new_map[new_y][x] = Place::Empty;
                    }
                }
            }
        }

        // Move all the south facing next
        for y in 0..ylen {
            for x in 0..xlen {
                if let Place::South = new_map[y][x] {
                    let mut new_x = x;
                    let mut new_y = y + 1;

                    if new_y == ylen {
                        new_y = 0;
                    }

                    // If this space is empty, move to it
                    if new_map[new_y][new_x] == Place::Empty {
                        new_map[new_y][new_x] = Place::South;
                        new_map[y][new_x] = Place::Empty;
                    }
                }
            }
        }

        // Draw the new map
        for y in 0..ylen {
            for x in 0..xlen {
                print!(
                    "{}",
                    match new_map[y][x] {
                        Place::East => '>',
                        Place::South => 'v',
                        Place::Empty => '.',
                    }
                );
            }
            println!();
        }
        println!();

        if new_map == old_map {
            break;
        }

        if i > 10 {
            break;
        }

        old_map = new_map.clone();
    }

    i
}

#[aoc(day25, part2)]
pub fn solve_part2(input: &[[Place; xlen]; ylen]) -> i32 {
    3
}
