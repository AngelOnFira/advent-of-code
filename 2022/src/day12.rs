use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

use itertools::Itertools;
use regex::Regex;

type InputType = HashMap<(i32, i32), Tile>;

#[derive(PartialEq)]
pub enum Tile {
    Start,
    End,
    Height(i32),
}

#[aoc_generator(day12)]
fn parse_input_day12(input: &str) -> InputType {
    // Chars
    // input.chars().collect()

    // Map to ints
    // input.lines().map(|x| x.parse::<i32>().unwrap()).collect()

    // Regex
    input
        .lines()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                acc.insert(
                    (x as i32, y as i32),
                    match c {
                        'S' => Tile::Start,
                        'E' => Tile::End,
                        _ => match c {
                            'a' => Tile::Height(0),
                            'b' => Tile::Height(1),
                            'c' => Tile::Height(2),
                            'd' => Tile::Height(3),
                            'e' => Tile::Height(4),
                            'f' => Tile::Height(5),
                            'g' => Tile::Height(6),
                            'h' => Tile::Height(7),
                            'i' => Tile::Height(8),
                            'j' => Tile::Height(9),
                            'k' => Tile::Height(10),
                            'l' => Tile::Height(11),
                            'm' => Tile::Height(12),
                            'n' => Tile::Height(13),
                            'o' => Tile::Height(14),
                            'p' => Tile::Height(15),
                            'q' => Tile::Height(16),
                            'r' => Tile::Height(17),
                            's' => Tile::Height(18),
                            't' => Tile::Height(19),
                            'u' => Tile::Height(20),
                            'v' => Tile::Height(21),
                            'w' => Tile::Height(22),
                            'x' => Tile::Height(23),
                            'y' => Tile::Height(24),
                            'z' => Tile::Height(25),
                            _ => Tile::Height(0),
                        },
                    },
                );
            });
            acc
        })
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &InputType) -> i32 {
    // Find the position of the start
    let start_pos = input
        .iter()
        .find(|(_, v)| **v == Tile::Start)
        .unwrap()
        .0
        .clone();

    // Find the position of the end
    let end_pos = input
        .iter()
        .find(|(_, v)| **v == Tile::End)
        .unwrap()
        .0
        .clone();

    // Find the shortest path from start to end, but moving to an adjacent tile
    // requires that it's at most 1 higher or lower than the current tile

    let mut pos = start_pos.clone();

    let mut explored: HashSet<(i32, i32)> = HashSet::new();

    let mut to_check = vec![(pos.clone(), 0, Vec::new())];
    explored.insert(pos.clone());

    while pos != end_pos {
        // Get the next tile to check
        let ((x, y), current_distance, path) = to_check.remove(0);

        // If we've arrived at the end, return the distance
        if (x, y) == end_pos {
            return current_distance;
        }

        // Update the pos
        pos = (x, y);

        // Find the adjacent tiles
        let adjacent_tiles = vec![
            (pos.0 - 1, pos.1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 - 1),
            (pos.0, pos.1 + 1),
        ];

        // Find the height of the current tile
        let current_height = match input.get(&pos) {
            Some(Tile::Height(h)) => *h,
            _ => 0,
        };

        // Find the adjacent tiles that are within 1 height of the current tile
        let adjacent_tiles = adjacent_tiles
            .iter()
            .filter(|(x, y)| {
                // If this tile is out of bounds, skip it
                if !input.contains_key(&(*x, *y)) {
                    return false;
                }

                let tile = input.get(&(*x, *y)).unwrap();

                match tile {
                    Tile::Height(h) => *h <= current_height || *h == current_height + 1,
                    Tile::End => current_height >= 24,
                    _ => false,
                }
            })
            .map(|(x, y)| (*x, *y))
            .collect_vec();

        // Add the adjacent tiles to the to_check list
        for tile in adjacent_tiles {
            // If we've already explored this tile, skip it
            if explored.contains(&tile) {
                continue;
            }

            // Add this to the explored list
            explored.insert(tile);

            let mut this_path = path.clone();
            this_path.push(tile);

            // Add to the to_check list
            to_check.push((tile, current_distance + 1, this_path));
        }
    }

    unreachable!()
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &InputType) -> i32 {
    // Find the position of the end
    let end_pos = input
        .iter()
        .find(|(_, v)| **v == Tile::End)
        .unwrap()
        .0
        .clone();

    // Find the shortest path from any position marked a
    input
        .iter()
        .filter(|(_, v)| **v == Tile::Height(0))
        .map(|(pos, _)| {
            let mut pos = *pos;

            let mut explored: HashSet<(i32, i32)> = HashSet::new();

            let mut to_check = vec![(pos.clone(), 0, Vec::new())];
            explored.insert(pos.clone());

            while pos != end_pos {
                // If we don't have a next tile, return a large number
                if to_check.is_empty() {
                    return 1000000;
                }

                // Get the next tile to check
                let ((x, y), current_distance, path) = to_check.remove(0);

                // If we've arrived at the end, return the distance
                if (x, y) == end_pos {
                    // Print the path
                    // println!(
                    //     "Path: {}",
                    //     path.iter()
                    //         .map(|(x, y)| format!("({}, {})", x, y))
                    //         .join(" -> ")
                    // );
                    return current_distance;
                }

                // Update the pos
                pos = (x, y);

                // Find the adjacent tiles
                let adjacent_tiles = vec![
                    (pos.0 - 1, pos.1),
                    (pos.0 + 1, pos.1),
                    (pos.0, pos.1 - 1),
                    (pos.0, pos.1 + 1),
                ];

                // Find the height of the current tile
                let current_height = match input.get(&pos) {
                    Some(Tile::Height(h)) => *h,
                    _ => 0,
                };

                // Find the adjacent tiles that are within 1 height of the current tile
                let adjacent_tiles = adjacent_tiles
                    .iter()
                    .filter(|(x, y)| {
                        // If this tile is out of bounds, skip it
                        if !input.contains_key(&(*x, *y)) {
                            return false;
                        }

                        let tile = input.get(&(*x, *y)).unwrap();

                        match tile {
                            Tile::Height(h) => *h <= current_height || *h == current_height + 1,
                            Tile::End => current_height >= 24,
                            _ => false,
                        }
                    })
                    .map(|(x, y)| (*x, *y))
                    .collect_vec();

                // Add the adjacent tiles to the to_check list
                for tile in adjacent_tiles {
                    // If we've already explored this tile, skip it
                    if explored.contains(&tile) {
                        continue;
                    }

                    // Add this to the explored list
                    explored.insert(tile);

                    let mut this_path = path.clone();
                    this_path.push(tile);

                    // Add to the to_check list
                    to_check.push((tile, current_distance + 1, this_path));
                }
            }

            unreachable!()
        })
        .min().unwrap()
}
