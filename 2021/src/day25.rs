use std::collections::HashMap;

use regex::Regex;

pub struct Instruction {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Place {
    East,
    South,
    Empty,
}

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> HashMap<(i32, i32), Place> {
    // v...>>.vv>
    // .vv>>.vv..
    // >>.>v>...v
    // >>v>>.>.v.
    // v>v.vv.v..
    // >.>>..v...
    // .vv..>.>v.
    // v.v..>>v.v
    // ....v..v.>

    let mut map = HashMap::new();

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            match c {
                '>' => map.insert((x as i32, y as i32), Place::East),
                'v' => map.insert((x as i32, y as i32), Place::South),
                '.' => map.insert((x as i32, y as i32), Place::Empty),
                _ => panic!("Unknown character: {}", c),
            };
        });
    });

    map
}

#[aoc(day25, part1)]
pub fn solve_part1(input: &HashMap<(i32, i32), Place>) -> i32 {
    let mut old_map = input.clone();

    let max_x = *old_map.keys().map(|(x, _)| x).max().unwrap();
    let min_x = *old_map.keys().map(|(x, _)| x).min().unwrap();
    let max_y = *old_map.keys().map(|(_, y)| y).max().unwrap();
    let min_y = *old_map.keys().map(|(_, y)| y).min().unwrap();

    let mut i = 0;
    loop {
        i += 1;
        let mut new_map = HashMap::new();

        // Move all the east facing first
        old_map.iter().for_each(|(k, v)| {
            if let Place::East = *v {
                let mut new_x = k.0 + 1;
                let mut new_y = k.1;

                // If were going over the edge, wrap around
                if new_x > max_x {
                    new_x = min_x;
                }

                let new_pos = (new_x, new_y);

                // If it's empty, move there
                if let Place::Empty = old_map.get(&new_pos).unwrap_or(&Place::Empty) {
                    new_map.insert(new_pos, Place::East)
                } else {
                    new_map.insert(*k, Place::East)
                };
            }
        });

        // Move all the south facing next

        new_map.clone().iter().for_each(|(k, v)| {
            if let Place::South = *v {
                let mut new_x = k.0;
                let mut new_y = k.1 + 1;

                // If were going over the edge, wrap around
                if new_y > max_y {
                    new_y = min_y;
                }

                let new_pos = (new_x, new_y);

                // If it's empty, move there
                if let Place::Empty = new_map.get(&new_pos).unwrap_or(&Place::Empty) {
                    new_map.insert(new_pos, Place::South)
                } else {
                    new_map.insert(*k, Place::South)
                };
            }
        });

        // Draw the new map
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let pos = (x, y);
                let place = new_map.get(&pos).unwrap_or(&Place::Empty);
                print!(
                    "{}",
                    match place {
                        Place::East => '>',
                        Place::South => 'v',
                        Place::Empty => '.',
                    }
                );
            }
            println!();
        }

        if new_map == old_map {
            break;
        }

        old_map = new_map;
    }

    i
}

#[aoc(day25, part2)]
pub fn solve_part2(input: &HashMap<(i32, i32), Place>) -> i32 {
    3
}
