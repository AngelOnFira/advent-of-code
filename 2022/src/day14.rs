use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

use itertools::Itertools;
use regex::Regex;

type InputType = HashMap<(i32, i32), bool>;

#[aoc_generator(day14)]
fn parse_input_day14(input: &str) -> InputType {
    // 497,29 -> 497,27 -> 497,29 -> 499,29 -> 499,25 -> 499,29 -> 501,29 -> 501,20 -> 501,29 -> 503,29 -> 503,19 -> 503,29 -> 505,29 -> 505,25 -> 505,29 -> 507,29 -> 507,24 -> 507,29
    // 477,77 -> 477,81 -> 473,81 -> 473,84 -> 484,84 -> 484,81 -> 483,81 -> 483,77

    // Regex
    let instructions = input
        .lines()
        .map(|x| {
            let re = Regex::new(r"(\d+),(\d+)").unwrap();
            let mut v = Vec::new();
            for cap in re.captures_iter(x) {
                v.push((
                    cap[1].parse::<i32>().unwrap(),
                    cap[2].parse::<i32>().unwrap(),
                ));
            }
            v
        })
        .collect::<Vec<_>>();

    // Turn all the paths into a map of filled tiles
    let mut map = HashMap::new();
    for path in instructions {
        for point in path.windows(2) {
            let (x1, y1) = point[0];
            let (x2, y2) = point[1];
            let (x1, x2) = (x1.min(x2), x1.max(x2));
            let (y1, y2) = (y1.min(y2), y1.max(y2));
            for x in x1..=x2 {
                for y in y1..=y2 {
                    map.insert((x, y), true);
                }
            }
        }
    }

    map
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &InputType) -> i32 {
    let sand_spawn = (500, 0);
    let mut sand = HashSet::new();

    // Start with the sand at the spawn point
    sand.insert(sand_spawn);

    // Let sand flow down. If it can't go down, try down and left. If it can't
    // go down and left, try down and right. If it can't go down and right, it
    // stops, and another piece of sand spawns. If a piece of sand goes on
    // forever, return the length of the sand hashset.

    let mut moving = true;
    let mut current_sand = sand_spawn;

    while moving {
        // println!("{:?}", current_sand);
        // If we've gone into the abyss, return the length of the sand hashset
        if current_sand.1 > 1000 {
            return sand.len() as i32 - 1;
        }

        // Try to go down
        let (x, y) = current_sand;
        let down = (x, y + 1);
        let left = (x - 1, y + 1);
        let right = (x + 1, y + 1);

        if !input.contains_key(&down) && !sand.contains(&down) {
            current_sand = down;
            continue;
        }

        if !input.contains_key(&left) && !sand.contains(&left) {
            // Try to go down and left
            current_sand = left;
            continue;
        }

        if !input.contains_key(&right) && !sand.contains(&right) {
            // Try to go down and right
            current_sand = right;
            continue;
        }

        // We can't go down, down and left, or down and right, so
        // we're stuck. Spawn a new piece of sand.
        sand.insert(current_sand);
        current_sand = sand_spawn;
        continue;
    }

    0
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &InputType) -> i32 {
    let sand_spawn = (500, 0);
    let mut sand = HashSet::new();

    // Find the greatest y position
    let max_y = input.iter().map(|x| x.0 .1).max().unwrap();

    // Let sand flow down. If it can't go down, try down and left. If it can't
    // go down and left, try down and right. If it can't go down and right, it
    // stops, and another piece of sand spawns. If a piece of sand goes on
    // forever, return the length of the sand hashset.

    let mut moving = true;
    let mut current_sand = sand_spawn;

    let mut last_loop = current_sand;

    while moving {
        // println!("{:?}", current_sand);

        // If sand contains the spawn, return the length of the sand hashset
        if sand.contains(&sand_spawn) {
            // Draw the sand map
            let (min_x, max_x) = sand
                .iter()
                .map(|x: &(i32, i32)| x.0)
                .minmax()
                .into_option()
                .unwrap();

            let (min_y, max_y) = sand
                .iter()
                .map(|x: &(i32, i32)| x.1)
                .minmax()
                .into_option()
                .unwrap();

            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    if sand.contains(&(x, y)) {
                        print!("#");
                    } else if input.contains_key(&(x, y)) {
                        print!("~");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }

            return sand.len() as i32;
        }

        // There is now a floor at y = 11, so we can stop there
        if current_sand.1 == max_y + 1 {
            sand.insert(current_sand);
            current_sand = sand_spawn;
            continue;
        }

        // Try to go down
        let (x, y) = current_sand;
        let down = (x, y + 1);
        let left = (x - 1, y + 1);
        let right = (x + 1, y + 1);

        if !input.contains_key(&down) && !sand.contains(&down) {
            current_sand = down;
            continue;
        }

        if !input.contains_key(&left) && !sand.contains(&left) {
            // Try to go down and left
            current_sand = left;
            continue;
        }

        if !input.contains_key(&right) && !sand.contains(&right) {
            // Try to go down and right
            current_sand = right;
            continue;
        }

        sand.insert(current_sand);
        current_sand = sand_spawn;
        continue;
    }

    0
}
